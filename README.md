# Units

## これは何

Rustでシンプルな単位系の型を実装してみたものです。  
`generic_const_exprs`の実装がまだ完璧ではないためか、[defs.rs](./src/defs.rs)のコメントアウトを外すとコンパイルに非常に時間がかかります。  
ラジアンなどの単位は実装されていません。四則演算以外の計算(累乗や累乗根)も実装されていません。  

## Acknowledgement

[uom](https://crates.io/crates/uom)
