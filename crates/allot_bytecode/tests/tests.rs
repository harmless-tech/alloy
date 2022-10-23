use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[test]
#[cfg(feature = "gen")]
#[cfg(feature = "parse")]
fn gen_parse() {
    todo!()
}
