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
  call void @llvm.dbg.declare(metadata i32* %res, metadata !15, metadata !DIExpression()), !dbg !17
  store i32 0, i32* %res, align 4, !dbg !17
  call void @llvm.dbg.declare(metadata i32* %i, metadata !18, metadata !DIExpression()), !dbg !20
  store i32 0, i32* %i, align 4, !dbg !20
  br label %for.cond, !dbg !21

for.cond:                                         ; preds = %for.inc6, %entry
  %0 = load i32, i32* %i, align 4, !dbg !22
  %cmp = icmp slt i32 %0, 10, !dbg !24
  br i1 %cmp, label %for.body, label %for.end8, !dbg !25

for.body:                                         ; preds = %for.cond
  call void @llvm.dbg.declare(metadata i32* %j, metadata !26, metadata !DIExpression()), !dbg !29
  store i32 0, i32* %j, align 4, !dbg !29
  br label %for.cond1, !dbg !30

for.cond1:                                        ; preds = %for.inc, %for.body
  %1 = load i32, i32* %j, align 4, !dbg !31
  %cmp2 = icmp slt i32 %1, 10, !dbg !33
  br i1 %cmp2, label %for.body3, label %for.end, !dbg !34

for.body3:                                        ; preds = %for.cond1
  call void @llvm.dbg.declare(metadata i32* %count, metadata !35, metadata !DIExpression()), !dbg !37
  store i32 0, i32* %count, align 4, !dbg !37
  br label %while.cond, !dbg !38

while.cond:                                       ; preds = %while.body, %for.body3
  %2 = load i32, i32* %count, align 4, !dbg !39
  %cmp4 = icmp slt i32 %2, 10, !dbg !40
  br i1 %cmp4, label %while.body, label %while.end, !dbg !38

while.body:                                       ; preds = %while.cond
  %3 = load i32, i32* %res, align 4, !dbg !41
  %add = add nsw i32 %3, 1, !dbg !41
  store i32 %add, i32* %res, align 4, !dbg !41
  %4 = load i32, i32* %count, align 4, !dbg !43
  %add5 = add nsw i32 %4, 1, !dbg !43
  store i32 %add5, i32* %count, align 4, !dbg !43
  br label %while.cond, !dbg !38, !llvm.loop !44

while.end:                                        ; preds = %while.cond
  br label %for.inc, !dbg !47

for.inc:                                          ; preds = %while.end
  %5 = load i32, i32* %j, align 4, !dbg !48
  %inc = add nsw i32 %5, 1, !dbg !48
  store i32 %inc, i32* %j, align 4, !dbg !48
  br label %for.cond1, !dbg !49, !llvm.loop !50

for.end:                                          ; preds = %for.cond1
  br label %for.inc6, !dbg !52

for.inc6:                                         ; preds = %for.end
  %6 = load i32, i32* %i, align 4, !dbg !53
  %inc7 = add nsw i32 %6, 1, !dbg !53
  store i32 %inc7, i32* %i, align 4, !dbg !53
  br label %for.cond, !dbg !54, !llvm.loop !55

for.end8:                                         ; preds = %for.cond
  %7 = load i32, i32* %res, align 4, !dbg !57
  ret i32 %7, !dbg !58
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6}
!llvm.ident = !{!7}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "examples/simple_loop.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "1aba08c0927bbfcd05c42fe91912fbd1")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"uwtable", i32 1}
!6 = !{i32 7, !"frame-pointer", i32 2}
!7 = !{!"clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)"}
!8 = distinct !DISubprogram(name: "main", scope: !9, file: !9, line: 4, type: !10, scopeLine: 5, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !14)
!9 = !DIFile(filename: "./examples/simple_loop.c", directory: "/home/woxjro/develop/tezos/lltz/front-end", checksumkind: CSK_MD5, checksum: "1aba08c0927bbfcd05c42fe91912fbd1")
!10 = !DISubroutineType(types: !11)
!11 = !{!12}
!12 = !DIDerivedType(tag: DW_TAG_typedef, name: "MyInt", file: !9, line: 1, baseType: !13)
!13 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!14 = !{}
!15 = !DILocalVariable(name: "res", scope: !8, file: !9, line: 6, type: !16)
!16 = !DIDerivedType(tag: DW_TAG_typedef, name: "Mutez", file: !9, line: 2, baseType: !13)
!17 = !DILocation(line: 6, column: 11, scope: !8)
!18 = !DILocalVariable(name: "i", scope: !19, file: !9, line: 7, type: !16)
!19 = distinct !DILexicalBlock(scope: !8, file: !9, line: 7, column: 5)
!20 = !DILocation(line: 7, column: 16, scope: !19)
!21 = !DILocation(line: 7, column: 10, scope: !19)
!22 = !DILocation(line: 7, column: 23, scope: !23)
!23 = distinct !DILexicalBlock(scope: !19, file: !9, line: 7, column: 5)
!24 = !DILocation(line: 7, column: 25, scope: !23)
!25 = !DILocation(line: 7, column: 5, scope: !19)
!26 = !DILocalVariable(name: "j", scope: !27, file: !9, line: 8, type: !16)
!27 = distinct !DILexicalBlock(scope: !28, file: !9, line: 8, column: 9)
!28 = distinct !DILexicalBlock(scope: !23, file: !9, line: 7, column: 36)
!29 = !DILocation(line: 8, column: 20, scope: !27)
!30 = !DILocation(line: 8, column: 14, scope: !27)
!31 = !DILocation(line: 8, column: 27, scope: !32)
!32 = distinct !DILexicalBlock(scope: !27, file: !9, line: 8, column: 9)
!33 = !DILocation(line: 8, column: 29, scope: !32)
!34 = !DILocation(line: 8, column: 9, scope: !27)
!35 = !DILocalVariable(name: "count", scope: !36, file: !9, line: 9, type: !16)
!36 = distinct !DILexicalBlock(scope: !32, file: !9, line: 8, column: 40)
!37 = !DILocation(line: 9, column: 19, scope: !36)
!38 = !DILocation(line: 10, column: 13, scope: !36)
!39 = !DILocation(line: 10, column: 20, scope: !36)
!40 = !DILocation(line: 10, column: 26, scope: !36)
!41 = !DILocation(line: 11, column: 21, scope: !42)
!42 = distinct !DILexicalBlock(scope: !36, file: !9, line: 10, column: 32)
!43 = !DILocation(line: 12, column: 23, scope: !42)
!44 = distinct !{!44, !38, !45, !46}
!45 = !DILocation(line: 13, column: 13, scope: !36)
!46 = !{!"llvm.loop.mustprogress"}
!47 = !DILocation(line: 14, column: 9, scope: !36)
!48 = !DILocation(line: 8, column: 35, scope: !32)
!49 = !DILocation(line: 8, column: 9, scope: !32)
!50 = distinct !{!50, !34, !51, !46}
!51 = !DILocation(line: 14, column: 9, scope: !27)
!52 = !DILocation(line: 15, column: 5, scope: !28)
!53 = !DILocation(line: 7, column: 31, scope: !23)
!54 = !DILocation(line: 7, column: 5, scope: !23)
!55 = distinct !{!55, !25, !56, !46}
!56 = !DILocation(line: 15, column: 5, scope: !19)
!57 = !DILocation(line: 16, column: 12, scope: !8)
!58 = !DILocation(line: 16, column: 5, scope: !8)
