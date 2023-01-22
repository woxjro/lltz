; ModuleID = './examples/simple_primitives.c'
source_filename = "./examples/simple_primitives.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [7 x i8] c"KT1...\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 !dbg !8 {
entry:
  %retval = alloca i32, align 4
  %addr = alloca i8*, align 8
  %amount = alloca i32, align 4
  %n = alloca i32, align 4
  %i = alloca i32, align 4
  store i32 0, i32* %retval, align 4
  call void @llvm.dbg.declare(metadata i8** %addr, metadata !14, metadata !DIExpression()), !dbg !18
  store i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.str, i64 0, i64 0), i8** %addr, align 8, !dbg !18
  call void @llvm.dbg.declare(metadata i32* %amount, metadata !19, metadata !DIExpression()), !dbg !21
  store i32 999, i32* %amount, align 4, !dbg !21
  call void @llvm.dbg.declare(metadata i32* %n, metadata !22, metadata !DIExpression()), !dbg !24
  store i32 77, i32* %n, align 4, !dbg !24
  call void @llvm.dbg.declare(metadata i32* %i, metadata !25, metadata !DIExpression()), !dbg !27
  store i32 -10, i32* %i, align 4, !dbg !27
  ret i32 0, !dbg !28
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6}
!llvm.ident = !{!7}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "examples/simple_primitives.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "a7c3005b6390e5aeab1f7eccca2d9b2e")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"uwtable", i32 1}
!6 = !{i32 7, !"frame-pointer", i32 2}
!7 = !{!"clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)"}
!8 = distinct !DISubprogram(name: "main", scope: !9, file: !9, line: 5, type: !10, scopeLine: 5, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !13)
!9 = !DIFile(filename: "./examples/simple_primitives.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "a7c3005b6390e5aeab1f7eccca2d9b2e")
!10 = !DISubroutineType(types: !11)
!11 = !{!12}
!12 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!13 = !{}
!14 = !DILocalVariable(name: "addr", scope: !8, file: !9, line: 6, type: !15)
!15 = !DIDerivedType(tag: DW_TAG_typedef, name: "_address", file: !9, line: 1, baseType: !16)
!16 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !17, size: 64)
!17 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!18 = !DILocation(line: 6, column: 14, scope: !8)
!19 = !DILocalVariable(name: "amount", scope: !8, file: !9, line: 7, type: !20)
!20 = !DIDerivedType(tag: DW_TAG_typedef, name: "_mutez", file: !9, line: 2, baseType: !12)
!21 = !DILocation(line: 7, column: 12, scope: !8)
!22 = !DILocalVariable(name: "n", scope: !8, file: !9, line: 8, type: !23)
!23 = !DIDerivedType(tag: DW_TAG_typedef, name: "_nat", file: !9, line: 4, baseType: !12)
!24 = !DILocation(line: 8, column: 10, scope: !8)
!25 = !DILocalVariable(name: "i", scope: !8, file: !9, line: 9, type: !26)
!26 = !DIDerivedType(tag: DW_TAG_typedef, name: "_int", file: !9, line: 3, baseType: !12)
!27 = !DILocation(line: 9, column: 10, scope: !8)
!28 = !DILocation(line: 10, column: 5, scope: !8)
