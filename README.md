# BakeFix 文字化けを直す

Shift_JIS による文字化けを直す WebAssembly ライブラリ。

(こちらのツール)[https://observablehq.com/@furidosu/bakefix]に使われてます。

文字化けの原理は、元エンコーデイングと違うエンコーディングでデコードしまうと、化けてる文字を読み込んで、
元エンコーディングで保存してしまうこと。

化けてる文字は違うエンコーディングでデコードした結果だから、違うエンコーディングでエンコードすれば、
できる限り元の状態を返せます。

このライブラリは、UTF-8で保存する文字をShift_JISで読み込んでしまう文字化けを直せます。
とはいえ、ただ文字をShift_JISでエンコードしてから読み直すだけです。
そのエンコード機能は encoding ライブラリが提供してくれます。

encoding が提供する WINDOWS31J は IBM の私有エリアコードを含めていないため、
ICU Converter Explorer に参照して補足します。

参考：

- [文字化けテスター](https://tools.m-bsys.com/development_tooles/char_corruption.php)
- [文字化け解説](https://tools.m-bsys.com/ex/mojibake_2.php)
- [ICU Converter Explorer ibm-943_P15A-2003](https://icu4c-demos.unicode.org/icu-bin/convexp?conv=ibm-943_P15A-2003)