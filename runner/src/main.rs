use anyhow::Context;
use std::path::PathBuf;
use wasmtime::component::{Component, Linker, ResourceTable, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

pub async fn add(add_path: PathBuf, calc_path: PathBuf, x: i32, y: i32) -> wasmtime::Result<i32> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut linker = Linker::<ServerWasiView>::new(&engine);

    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    let wasi_view = ServerWasiView::new(wasi);
    let mut store = Store::new(&engine, wasi_view);

    let add_component =
        Component::from_file(&engine, add_path).context("Add component file not found")?;
    let calc_component =
        Component::from_file(&engine, calc_path).context("Calculator component file not found")?;

    let add_instance = linker.instantiate_async(&mut store, &add_component).await?;

    let add = add_instance
        .exports(&mut store)
        .instance("docs:adder/add")
        .expect("instance 'docs:adder/add' not found")
        .func("add")
        .expect("add function not found");

    // let mut results = [Val::S32(0)];

    // add.call_async(&mut store, &[Val::S32(5), Val::S32(3)], &mut results)
    //     .await?;

    // let Val::S32(result) = results[0] else {
    //     panic!("Unexpected result type");
    // };

    // println!("5 + 3 = {}", result);

    linker.instance("docs:adder/add")?.func_wrap_async(
        "add",
        move |store, params: (i32, i32)| {
            Box::new(async move {
                let mut results = [Val::S32(0)];
                add.call_async(
                    store,
                    &[Val::S32(params.0), Val::S32(params.1)],
                    &mut results,
                )
                .await?;

                if let Val::S32(result) = results[0] {
                    Ok((result,))
                } else {
                    Err(anyhow::anyhow!("Unexpected result type"))
                }
            })
        },
    )?;

    let calc_instance = linker
        .instantiate_async(&mut store, &calc_component)
        .await?;
    let calc_fn = calc_instance
        .exports(&mut store)
        .instance("component:calculator/calculate")
        .expect("instance 'component:calculator/calculate' not found")
        .func("eval-expression")
        .expect("eval-expression function not found");
    let mut result = [Val::S32(0)];
    calc_fn
        .call_async(
            &mut store,
            &[Val::String(format!("{} + {}", x, y))],
            &mut result,
        )
        .await?;
    let Val::S32(result) = result[0] else {
        panic!("Unexpected result type");
    };
    Ok(result)
}

struct ServerWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new(ctx: WasiCtx) -> Self {
        let table = ResourceTable::new();
        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let add_path = PathBuf::from("target/wasm32-wasip1/release/add.wasm");
    let calc_path = PathBuf::from("target/wasm32-wasip1/release/calculator.wasm");
    let lhs = 10;
    let rhs = 20;
    let sum = add(add_path, calc_path, lhs, rhs).await?;
    println!("{} + {} = {sum}", lhs, rhs);
    Ok(())
}
