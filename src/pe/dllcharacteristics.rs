/*
常量	Value	说明
0x0001
保留，必须为零。
0x0002
保留，必须为零。
0x0004
保留，必须为零。
0x0008
保留，必须为零。
IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA
0x0020
映像可处理高熵 64 位虚拟地址空间。
IMAGE_DLLCHARACTERISTICS_
DYNAMIC_BASE
0x0040
DLL 可以在加载时重定位。
IMAGE_DLLCHARACTERISTICS_
FORCE_INTEGRITY
0x0080
强制实施了代码完整性检查。
IMAGE_DLLCHARACTERISTICS_
NX_COMPAT
0x0100
该映像与 NX 兼容。
IMAGE_DLLCHARACTERISTICS_ NO_ISOLATION
0x0200
隔离感知，但不隔离映像。
IMAGE_DLLCHARACTERISTICS_ NO_SEH
0x0400
不使用结构化异常 (SE) 处理。 该映像中无法调用任何 SE 处理程序。
IMAGE_DLLCHARACTERISTICS_ NO_BIND
0x0800
请勿绑定此映像。
IMAGE_DLLCHARACTERISTICS_APPCONTAINER
0x1000
映像必须在 AppContainer 中执行。
IMAGE_DLLCHARACTERISTICS_ WDM_DRIVER
0x2000
WDM 驱动程序。
IMAGE_DLLCHARACTERISTICS_GUARD_CF
0x4000
映像支持控制流保护。
IMAGE_DLLCHARACTERISTICS_ TERMINAL_SERVER_AWARE
0x8000
终端服务器感知。
*/
pub fn get_dllcharacteristics_text(machine: u16) -> &'static str {
match machine {
    0x0001 => "保留，必须为零。",
    0x0002 => "保留，必须为零。",
    0x0004 => "保留，必须为零。",
    0x0008 => "保留，必须为零。",
    0x0020 => "映像可处理高熵 64 位虚拟地址空间。",
    0x0040 => "DLL 可以在加载时重定位。",
    0x0080 => "强制实施了代码完整性检查。",
    0x0100 => "该映像与 NX 兼容。",
    0x0200 => "隔离感知，但不隔离映像。",
    0x0400 => "不使用结构化异常 (SE) 处理。 该映像中无法调用任何 SE 处理程序。",
    0x0800 => "请勿绑定此映像。",
    0x1000 => "映像必须在 AppContainer 中执行。",
    0x2000 => "WDM 驱动程序。",
    0x4000 => "映像支持控制流保护。",
    0x8000 => "终端服务器感知。",
    _ => "未知",
    
}
}
