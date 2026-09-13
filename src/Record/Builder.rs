pub fn Record_Builder_copyRecord(record: crate::UnknownType) -> crate::UnknownType {
    // Builder inserts must retain call order even when the generic carrier has
    // predeclared slots for those labels. Copy children shallowly into an ordered
    // dynamic record; subsequent updates retain COW isolation from the input.
    let fields = record.__purust_record_fields().expect("Expected record");
    crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(fields))
}

pub fn Record_Builder_unsafeInsert(
    key: String,
    value: crate::UnknownType,
    record: crate::UnknownType,
) -> crate::UnknownType {
    // The optimizer may remove copyRecord on a fresh empty literal.
    let record = if matches!(record.resolve(), crate::Value::DynamicRecord(_)) {
        record
    } else {
        Record_Builder_copyRecord(record)
    };
    record.__purust_set_field(&key, value)
}
