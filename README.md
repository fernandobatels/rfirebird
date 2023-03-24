# rfirebird - Firebird tool for raw access the database files

[![Crate](https://img.shields.io/crates/v/rfirebird.svg)](https://crates.io/crates/rfirebird)
[![API](https://docs.rs/rfirebird/badge.svg)](https://docs.rs/rfirebird)
[![github sponsors](https://img.shields.io/github/sponsors/fernandobatels)](https://github.com/sponsors/fernandobatels)

This is a study and demonstration project. Only use this project with offline copy of your database file.

## Examples

Tables of a database
``` bash
cargo run tables dbs/employee.fdb
 name                     | is_system_table | relation
--------------------------+-----------------+----------
 RDB$PAGES                | true            | 0
 RDB$DATABASE             | true            | 1
 RDB$FIELDS               | true            | 2
 RDB$INDEX_SEGMENTS       | true            | 3
```

Columns of a table
``` bash
cargo run columns dbs/employee.fdb sales
 position | name         | size | type      | scale | is_not_null | is_computed
----------+--------------+------+-----------+-------+-------------+-------------
 0        | PO_NUMBER    | 8    | Char      | 0     | true        | false
 1        | CUST_NO      | 4    | Integer   | 0     | true        | false
 2        | SALES_REP    | 2    | Smallint  | 0     | false       | false
```

Values of a table
``` bash
cargo run rows dbs/employee.fdb customer
 CUST_NO | CUSTOMER                  | CONTACT_FIRST | CONTACT_LAST  | PHONE_NO        | ADDRESS_LINE1               | ADDRESS_LINE2 | CITY              | STATE_PROVINCE | COUNTRY     | POSTAL_CODE | ON_HOLD
---------+---------------------------+---------------+---------------+-----------------+-----------------------------+---------------+-------------------+----------------+-------------+-------------+---------
 1001    | Signature Design          | Dale J.       | Little        | (619) 530-2710  | 15500 Pacific Heights Blvd. |               | San Diego         | CA             | USA         | 92121       |
 1002    | Dallas Technologies       | Glen          | Brown         | (214) 960-2233  | P. O. Box 47000             |               | Dallas            | TX             | USA         | 75205       | *
```

## Goals

- [x] Open database files
- [x] Access tables
- [x] Read data rows
- [ ] Handle big database files
- [ ] Support firebird 1.0 files
- [ ] Support firebird 2.0 files
- [x] Support firebird 3.0 files
- [ ] Support firebird 4.0 files

Types
- [x] Varchar
- [x] Char
- [x] Int
- [x] SmallInt
- [ ] Float
- [ ] Decimal
- [ ] Numeric
- [ ] Timestamp
- [ ] Date
- [ ] Time

CLI
- [x] Open .fdb files
- [x] List tables
- [x] Show records of a table

## References

- https://firebirdsql.org/file/documentation/html/en/firebirddocs/firebirdinternals/firebird-internals.html#fbint-introduction
- https://firebirdsql.org/manual/fbint-structure.html
- https://ib-aid.com/download/docs/firebird-language-reference-2.5/fblangref-appx04-fields.html
