table "users" {
  schema = schema.example
  column "id" {
    null = false
    type = int
  }
  column "name" {
    null = true
    type = varchar(100)
  }
  primary_key {
    columns = [column.id]
  }
}
