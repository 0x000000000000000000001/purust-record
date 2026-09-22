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

pub fn Record_Builder_unsafeModify(
    key: String,
    update: purust_core::Func1<crate::UnknownType, crate::UnknownType>,
    record: crate::UnknownType,
) -> crate::UnknownType {
    let current = record
        .__purust_get_field(&key)
        .expect("Record.Builder.unsafeModify: missing field");
    let updated = update(current);
    let record = if matches!(record.resolve(), crate::Value::DynamicRecord(_)) {
        record
    } else {
        Record_Builder_copyRecord(record)
    };
    record.__purust_set_field(&key, updated)
}

pub fn Record_Builder_unsafeDelete(key: String, record: crate::UnknownType) -> crate::UnknownType {
    let mut fields = record
        .__purust_record_fields()
        .expect("Record.Builder.unsafeDelete: expected record");
    fields.remove(&key);
    crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(fields))
}

pub fn Record_Builder_unsafeRename(
    from: String,
    to: String,
    record: crate::UnknownType,
) -> crate::UnknownType {
    let mut fields = record
        .__purust_record_fields()
        .expect("Record.Builder.unsafeRename: expected record");
    if let Some(value) = fields.remove(&from) {
        fields.insert(to, value);
    }
    crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(fields))
}
