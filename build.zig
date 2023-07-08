// © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
// License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

const std = @import("std");
const Target = @import("std").Target;
const CrossTarget = @import("std").zig.CrossTarget;
const Feature = @import("std").Target.Cpu.Feature;

pub fn build(b: *std.build.Builder) void {
    const features = Target.riscv.Feature;

    var enabled_features = Feature.Set.empty;
    enabled_features.addFeature(@enumToInt(features.m));
    enabled_features.addFeature(@enumToInt(features.a));
    enabled_features.addFeature(@enumToInt(features.c));

    const target = CrossTarget{
        .cpu_arch = Target.Cpu.Arch.riscv64,
        .os_tag = Target.Os.Tag.freestanding,
        .abi = Target.Abi.none,
        .cpu_features_add = enabled_features,
    };

    const kern = b.addExecutable("GeNT-kern", "src/main.zig");
    kern.addAssemblyFileSource(.{ .path = "src/init.S" });
    kern.setLinkerScriptPath(.{ .path = "config/linker.lds" });
    kern.code_model = .small;
    kern.target = target;

    b.installArtifact(kern);
}
