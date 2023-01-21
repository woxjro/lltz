; ModuleID = './examples/simple_if.c'
source_filename = "./examples/simple_if.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main(i32 noundef %argc, i8** noundef %argv) #0 !dbg !8 {
entry:
  %retval = alloca i32, align 4
  %argc.addr = alloca i32, align 4
  %argv.addr = alloca i8**, align 8
  %a = alloca i32, align 4
  store i32 0, i32* %retval, align 4
  store i32 %argc, i32* %argc.addr, align 4
  call void @llvm.dbg.declare(metadata i32* %argc.addr, metadata !17, metadata !DIExpression()), !dbg !18
  store i8** %argv, i8*** %argv.addr, align 8
  call void @llvm.dbg.declare(metadata i8*** %argv.addr, metadata !19, metadata !DIExpression()), !dbg !20
  call void @llvm.dbg.declare(metadata i32* %a, metadata !21, metadata !DIExpression()), !dbg !22
  store i32 10, i32* %a, align 4, !dbg !22
  %0 = load i32, i32* %a, align 4, !dbg !23
  %cmp = icmp slt i32 %0, 5, !dbg !25
  br i1 %cmp, label %if.then, label %if.else, !dbg !26

if.then:                                          ; preds = %entry
  %1 = load i32, i32* %a, align 4, !dbg !27
  %add = add nsw i32 %1, 100, !dbg !27
  store i32 %add, i32* %a, align 4, !dbg !27
  br label %if.end, !dbg !29

if.else:                                          ; preds = %entry
  %2 = load i32, i32* %a, align 4, !dbg !30
  %sub = sub nsw i32 %2, 100, !dbg !30
  store i32 %sub, i32* %a, align 4, !dbg !30
  br label %if.end

if.end:                                           ; preds = %if.else, %if.then
  %3 = load i32, i32* %a, align 4, !dbg !32
  ret i32 %3, !dbg !33
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6}
!llvm.ident = !{!7}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "examples/simple_if.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "bacd1f5d06308244ba441a60699c8232")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"uwtable", i32 1}
!6 = !{i32 7, !"frame-pointer", i32 2}
!7 = !{!"clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)"}
!8 = distinct !DISubprogram(name: "main", scope: !9, file: !9, line: 1, type: !10, scopeLine: 2, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !16)
!9 = !DIFile(filename: "./examples/simple_if.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "bacd1f5d06308244ba441a60699c8232")
!10 = !DISubroutineType(types: !11)
!11 = !{!12, !12, !13}
!12 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!13 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !14, size: 64)
!14 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !15, size: 64)
!15 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!16 = !{}
!17 = !DILocalVariable(name: "argc", arg: 1, scope: !8, file: !9, line: 1, type: !12)
!18 = !DILocation(line: 1, column: 14, scope: !8)
!19 = !DILocalVariable(name: "argv", arg: 2, scope: !8, file: !9, line: 1, type: !13)
!20 = !DILocation(line: 1, column: 26, scope: !8)
!21 = !DILocalVariable(name: "a", scope: !8, file: !9, line: 3, type: !12)
!22 = !DILocation(line: 3, column: 9, scope: !8)
!23 = !DILocation(line: 4, column: 9, scope: !24)
!24 = distinct !DILexicalBlock(scope: !8, file: !9, line: 4, column: 9)
!25 = !DILocation(line: 4, column: 11, scope: !24)
!26 = !DILocation(line: 4, column: 9, scope: !8)
!27 = !DILocation(line: 5, column: 11, scope: !28)
!28 = distinct !DILexicalBlock(scope: !24, file: !9, line: 4, column: 16)
!29 = !DILocation(line: 6, column: 5, scope: !28)
!30 = !DILocation(line: 7, column: 11, scope: !31)
!31 = distinct !DILexicalBlock(scope: !24, file: !9, line: 6, column: 12)
!32 = !DILocation(line: 9, column: 12, scope: !8)
!33 = !DILocation(line: 9, column: 5, scope: !8)
