pub fn Record_Unsafe_Union_unsafeUnionFn() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(std::rc::Rc::new(
        |first, second| {
            let mut fields = second
                .__purust_record_fields()
                .expect("Record.Unsafe.Union: expected record");
            for (key, value) in first
                .__purust_record_fields()
                .expect("Record.Unsafe.Union: expected record")
                .entries()
            {
                // The first argument wins, like the JS FFI.
                fields.insert(key, value);
            }
            crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(fields))
        },
    )))
}
