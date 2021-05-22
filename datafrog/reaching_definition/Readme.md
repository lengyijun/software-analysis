use datafrog to implement reaching definition

https://www.bilibili.com/video/av95400721/?spm_id_from=333.788.b_765f64657363.3

一个definition在某一点是否还有效，在中间是不是被重定义了。

在程序开头给一个dummy definition。在使用时，如果dummy definition能reach，表明这个变量可能还没有被定义过，需要报错

may analyze

// 只关注=左边的变量，即a
// 不关注=右边的b
a=b+1

视频里以block为单位，但是我的实现以语句为单位。也可以看做每个语句都是一个block

```
  a=....;
p->
```

reaching_definition.pl: prolog version of reaching definition. but fail to run, and I can't debug it now.
