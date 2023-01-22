; ModuleID = './examples/simple_loop.c'
source_filename = "./examples/simple_loop.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 !dbg !8 {
entry:
  %retval = alloca i32, align 4
  %res = alloca i32, align 4
  %i = alloca i32, align 4
  %j = alloca i32, align 4
  %count = alloca i32, align 4
  store i32 0, i32* %retval, align 4
  call void @llvm.dbg.declare(metadata i32* %res, metadata !14, metadata !DIExpression()), !dbg !16
  store i32 0, i32* %res, align 4, !dbg !16
  call void @llvm.dbg.declare(metadata i32* %i, metadata !17, metadata !DIExpression()), !dbg !19
  store i32 0, i32* %i, align 4, !dbg !19
  br label %for.cond, !dbg !20

for.cond:                                         ; preds = %for.inc6, %entry
  %0 = load i32, i32* %i, align 4, !dbg !21
  %cmp = icmp slt i32 %0, 10, !dbg !23
  br i1 %cmp, label %for.body, label %for.end8, !dbg !24

for.body:                                         ; preds = %for.cond
  call void @llvm.dbg.declare(metadata i32* %j, metadata !25, metadata !DIExpression()), !dbg !28
  store i32 0, i32* %j, align 4, !dbg !28
  br label %for.cond1, !dbg !29

for.cond1:                                        ; preds = %for.inc, %for.body
  %1 = load i32, i32* %j, align 4, !dbg !30
  %cmp2 = icmp slt i32 %1, 10, !dbg !32
  br i1 %cmp2, label %for.body3, label %for.end, !dbg !33

for.body3:                                        ; preds = %for.cond1
  call void @llvm.dbg.declare(metadata i32* %count, metadata !34, metadata !DIExpression()), !dbg !36
  store i32 0, i32* %count, align 4, !dbg !36
  br label %while.cond, !dbg !37

while.cond:                                       ; preds = %while.body, %for.body3
  %2 = load i32, i32* %count, align 4, !dbg !38
  %cmp4 = icmp slt i32 %2, 10, !dbg !39
  br i1 %cmp4, label %while.body, label %while.end, !dbg !37

while.body:                                       ; preds = %while.cond
  %3 = load i32, i32* %res, align 4, !dbg !40
  %add = add nsw i32 %3, 1, !dbg !40
  store i32 %add, i32* %res, align 4, !dbg !40
  %4 = load i32, i32* %count, align 4, !dbg !42
  %add5 = add nsw i32 %4, 1, !dbg !42
  store i32 %add5, i32* %count, align 4, !dbg !42
  br label %while.cond, !dbg !37, !llvm.loop !43

while.end:                                        ; preds = %while.cond
  br label %for.inc, !dbg !46

for.inc:                                          ; preds = %while.end
  %5 = load i32, i32* %j, align 4, !dbg !47
  %inc = add nsw i32 %5, 1, !dbg !47
  store i32 %inc, i32* %j, align 4, !dbg !47
  br label %for.cond1, !dbg !48, !llvm.loop !49

for.end:                                          ; preds = %for.cond1
  br label %for.inc6, !dbg !51

for.inc6:                                         ; preds = %for.end
  %6 = load i32, i32* %i, align 4, !dbg !52
  %inc7 = add nsw i32 %6, 1, !dbg !52
  store i32 %inc7, i32* %i, align 4, !dbg !52
  br label %for.cond, !dbg !53, !llvm.loop !54

for.end8:                                         ; preds = %for.cond
  %7 = load i32, i32* %res, align 4, !dbg !56
  ret i32 %7, !dbg !57
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6}
!llvm.ident = !{!7}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "examples/simple_loop.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "23daa12da8e5e2f11c9baa921a6acaab")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"uwtable", i32 1}
!6 = !{i32 7, !"frame-pointer", i32 2}
!7 = !{!"clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)"}
!8 = distinct !DISubprogram(name: "main", scope: !9, file: !9, line: 3, type: !10, scopeLine: 4, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !13)
!9 = !DIFile(filename: "./examples/simple_loop.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "23daa12da8e5e2f11c9baa921a6acaab")
!10 = !DISubroutineType(types: !11)
!11 = !{!12}
!12 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!13 = !{}
!14 = !DILocalVariable(name: "res", scope: !8, file: !9, line: 5, type: !15)
!15 = !DIDerivedType(tag: DW_TAG_typedef, name: "_mutez", file: !9, line: 1, baseType: !12)
!16 = !DILocation(line: 5, column: 12, scope: !8)
!17 = !DILocalVariable(name: "i", scope: !18, file: !9, line: 6, type: !15)
!18 = distinct !DILexicalBlock(scope: !8, file: !9, line: 6, column: 5)
!19 = !DILocation(line: 6, column: 17, scope: !18)
!20 = !DILocation(line: 6, column: 10, scope: !18)
!21 = !DILocation(line: 6, column: 24, scope: !22)
!22 = distinct !DILexicalBlock(scope: !18, file: !9, line: 6, column: 5)
!23 = !DILocation(line: 6, column: 26, scope: !22)
!24 = !DILocation(line: 6, column: 5, scope: !18)
!25 = !DILocalVariable(name: "j", scope: !26, file: !9, line: 7, type: !15)
!26 = distinct !DILexicalBlock(scope: !27, file: !9, line: 7, column: 9)
!27 = distinct !DILexicalBlock(scope: !22, file: !9, line: 6, column: 37)
!28 = !DILocation(line: 7, column: 21, scope: !26)
!29 = !DILocation(line: 7, column: 14, scope: !26)
!30 = !DILocation(line: 7, column: 28, scope: !31)
!31 = distinct !DILexicalBlock(scope: !26, file: !9, line: 7, column: 9)
!32 = !DILocation(line: 7, column: 30, scope: !31)
!33 = !DILocation(line: 7, column: 9, scope: !26)
!34 = !DILocalVariable(name: "count", scope: !35, file: !9, line: 8, type: !15)
!35 = distinct !DILexicalBlock(scope: !31, file: !9, line: 7, column: 41)
!36 = !DILocation(line: 8, column: 20, scope: !35)
!37 = !DILocation(line: 9, column: 13, scope: !35)
!38 = !DILocation(line: 9, column: 20, scope: !35)
!39 = !DILocation(line: 9, column: 26, scope: !35)
!40 = !DILocation(line: 10, column: 21, scope: !41)
!41 = distinct !DILexicalBlock(scope: !35, file: !9, line: 9, column: 32)
!42 = !DILocation(line: 11, column: 23, scope: !41)
!43 = distinct !{!43, !37, !44, !45}
!44 = !DILocation(line: 12, column: 13, scope: !35)
!45 = !{!"llvm.loop.mustprogress"}
!46 = !DILocation(line: 13, column: 9, scope: !35)
!47 = !DILocation(line: 7, column: 36, scope: !31)
!48 = !DILocation(line: 7, column: 9, scope: !31)
!49 = distinct !{!49, !33, !50, !45}
!50 = !DILocation(line: 13, column: 9, scope: !26)
!51 = !DILocation(line: 14, column: 5, scope: !27)
!52 = !DILocation(line: 6, column: 32, scope: !22)
!53 = !DILocation(line: 6, column: 5, scope: !22)
!54 = distinct !{!54, !24, !55, !45}
!55 = !DILocation(line: 14, column: 5, scope: !18)
!56 = !DILocation(line: 15, column: 12, scope: !8)
!57 = !DILocation(line: 15, column: 5, scope: !8)
