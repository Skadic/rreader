fn main() {
    let src = [
        "external/repair/array.c",
        "external/repair/basics.c",
        "external/repair/hash.c",
        "external/repair/heap.c",
        "external/repair/records.c",
        "external/repair/repair.c",
    ];

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("external/repair")
        .flag("-Wno-unused-variable")
        .flag("-Wno-sign-compare");

    build.compile("repair");
}
