raw: don't use datafrog

datafrog: use datafrog

---

Rust中borrow check检查有两种实现，一种是stable中的NLL，另一种是nightly中的polonius。polonius相比于NLL能覆盖一些corner case

cargo  rustc  -- -Zpolonius

polonius的实现思路是基于prolog的，不过当然不能直接写prolog代码，而是用了一个叫datafrog的crate

datafrog实现了类似prolog的功能。你可以定义一些静态fact（Relation），动态fact（Variable）和他们之间的运算规则，datafrog就会一直计算，扩充fact。fact只能增加，不能减少。datafrog发现fact不再增加，就会停止

于是我用datafrog实现了南京大学静态程序分析课的里reach definition和live analysis两个算法。感觉与不用datafrog直接写相比，用了datafrog只要专注于定好Rule就行了。不用datafrog还要考虑计算的细节，较为繁琐

使用datafrog也能帮助你思考，哪些是常量，可以一上来就计算好（datafrog中叫做Relation），哪些是动态计算的（datafrog中叫做Variable）

其实我还用prolog也写了一遍，但是用swipl执行的时候有些问题，也不太会修，就放弃了


