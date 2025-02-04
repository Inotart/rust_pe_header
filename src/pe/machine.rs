/// 定义一个公共函数 `get_machine_type`，它接受一个 `u16` 类型的参数 `machine`，并返回一个静态字符串引用
/*
https://learn.microsoft.com/zh-cn/windows/win32/debug/pe-format
*/
pub fn get_machine_type(machine: u16) -> &'static str {
    match machine {
        0x0=>"UNKNOWN",
        0x184=>"ALPHA",
        0x284=>"ALPHA64",
        0x1d3=>"AM33",
        0x8664=>"AMD64",
        0x1c0=>"ARM",
        0xaa64=>"ARM64",
        0x1c4=>"ARMNT",
        0xebc=>"EBC",
        0x14c=>"I386",
        0x200=>"IA64",
        0x6232=>"LOONGARCH32",
        0x6264=>"LOONGARCH64",
        0x9041=>"M32R",
        0x266=>"MIPS16",
        0x366=>"MIPSFPU",
        0x466=>"MIPSFPU16",
        0x1f0=>"POWERPC",
        0x1f1=>"POWERPCFP",
        0x166=>"R4000",
        0x5032=>"RISCV32",
        0x5064=>"RISCV64",
        0x5128=>"RISCV128",
        0x1a2=>"SH3",
        0x1a3=>"SH3DSP",
        0x1a6=>"SH4",
        0x1a8=>"SH5",
        0x1c2=>"THUMB",
        0x169=>"WCEMIPSV2",
        _ => "Unknown machine type,maybe you should check the PE format specification",
        
    }
    
}
// 判断32位或64位系统,值含义如下
// 0:未知
// 1:32位
// 2:64位

pub fn get_machine_is32_or64(machine: u16) -> u8 {
    match machine {
        0x0=>0,
        0x14c=>1,
        0x1c0=>1,
        0x8664=>2,
        0xaa64=>2,
        // 这里有待补充,我这里就不写了哈，坐等pr - 我不是art

        _ => 0,
    }
    
}