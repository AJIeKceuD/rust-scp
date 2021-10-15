Basic Rules:
- Each column got own type like Option<> (example: RequestId(Option<i64>))
- ColumnType::None mean Null in DB. None without type mean dont change column. Like: Option<RequestId(Option<i64>)>

SQLx;
- Use reversible migrations:
$ sqlx migrate add -r <name>
- Apply migrations:
$ sqlx migrate run