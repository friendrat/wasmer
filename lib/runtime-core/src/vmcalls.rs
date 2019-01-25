use crate::{
    memory::{DynamicMemory, StaticMemory},
    structures::TypedIndex,
    types::{ImportedMemoryIndex, LocalMemoryIndex, LocalTableIndex},
    vm,
};

// +*****************************+
// |       LOCAL MEMORIES        |
// +****************************+

pub unsafe extern "C" fn local_static_memory_grow(
    memory_index: LocalMemoryIndex,
    delta: u32,
    ctx: &mut vm::Ctx,
) -> i32 {
    let local_memory = *ctx.memories.add(memory_index.index());
    let memory = (*local_memory).memory as *mut StaticMemory;

    if let Some(old) = (*memory).grow(delta, &mut *local_memory) {
        old as i32
    } else {
        -1
    }
}

pub unsafe extern "C" fn local_static_memory_size(
    memory_index: LocalMemoryIndex,
    ctx: &vm::Ctx,
) -> u32 {
    let local_memory = *ctx.memories.add(memory_index.index());
    let memory = (*local_memory).memory as *mut StaticMemory;

    (*memory).current()
}

pub unsafe extern "C" fn local_dynamic_memory_grow(
    memory_index: LocalMemoryIndex,
    delta: u32,
    ctx: &mut vm::Ctx,
) -> i32 {
    let local_memory = *ctx.memories.add(memory_index.index());
    let memory = (*local_memory).memory as *mut DynamicMemory;

    if let Some(old) = (*memory).grow(delta, &mut *local_memory) {
        old as i32
    } else {
        -1
    }
}

pub unsafe extern "C" fn local_dynamic_memory_size(
    memory_index: LocalMemoryIndex,
    ctx: &vm::Ctx,
) -> u32 {
    let local_memory = *ctx.memories.add(memory_index.index());
    let memory = (*local_memory).memory as *mut DynamicMemory;

    (*memory).current()
}

// +*****************************+
// |      IMPORTED MEMORIES      |
// +****************************+

pub unsafe extern "C" fn imported_static_memory_grow(
    import_memory_index: ImportedMemoryIndex,
    delta: u32,
    ctx: &mut vm::Ctx,
) -> i32 {
    let local_memory = *ctx.imported_memories.add(import_memory_index.index());
    let memory = (*local_memory).memory as *mut StaticMemory;

    if let Some(old) = (*memory).grow(delta, &mut *local_memory) {
        old as i32
    } else {
        -1
    }
}

pub unsafe extern "C" fn imported_static_memory_size(
    import_memory_index: ImportedMemoryIndex,
    ctx: &vm::Ctx,
) -> u32 {
    let local_memory = *ctx.imported_memories.add(import_memory_index.index());
    let memory = (*local_memory).memory as *mut StaticMemory;

    (*memory).current()
}

pub unsafe extern "C" fn imported_dynamic_memory_grow(
    memory_index: ImportedMemoryIndex,
    delta: u32,
    ctx: &mut vm::Ctx,
) -> i32 {
    let local_memory = *ctx.imported_memories.add(memory_index.index());
    let memory = (*local_memory).memory as *mut DynamicMemory;

    if let Some(old) = (*memory).grow(delta, &mut *local_memory) {
        old as i32
    } else {
        -1
    }
}

pub unsafe extern "C" fn imported_dynamic_memory_size(
    memory_index: ImportedMemoryIndex,
    ctx: &vm::Ctx,
) -> u32 {
    let local_memory = *ctx.imported_memories.add(memory_index.index());
    let memory = (*local_memory).memory as *mut DynamicMemory;

    (*memory).current()
}

// +*****************************+
// |        LOCAL TABLES         |
// +****************************+

pub unsafe extern "C" fn local_table_grow(
    table_index: LocalTableIndex,
    delta: u32,
    ctx: &mut vm::Ctx,
) -> i32 {
    let _ = table_index;
    let _ = delta;
    let _ = ctx;
    unimplemented!()
}

pub unsafe extern "C" fn local_table_size(table_index: LocalTableIndex, ctx: &vm::Ctx) -> u32 {
    let _ = table_index;
    let _ = ctx;
    unimplemented!()
}
