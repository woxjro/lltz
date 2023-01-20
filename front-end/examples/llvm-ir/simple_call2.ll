; ModuleID = './examples/llvm-ir/simple_call.ll'
source_filename = "./examples/simple_call.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @func(i32 noundef %a, i32 noundef %b) #0 {
entry:
  %a.addr = alloca i32, align 4
  %b.addr = alloca i32, align 4
  %res = alloca i32, align 4
  store i32 %a, i32* %a.addr, align 4
  store i32 %b, i32* %b.addr, align 4
  store i32 0, i32* %res, align 4
  %0 = load i32, i32* %a.addr, align 4
  %cmp = icmp slt i32 %0, 10
  br i1 %cmp, label %if.then, label %if.else

if.then:                                          ; preds = %entry
  %1 = load i32, i32* %a.addr, align 4
  %2 = load i32, i32* %b.addr, align 4
  %add = add nsw i32 %1, %2
  %3 = load i32, i32* %res, align 4
  %add1 = add nsw i32 %3, %add
  store i32 %add1, i32* %res, align 4
  br label %if.end

if.else:                                          ; preds = %entry
  %4 = load i32, i32* %a.addr, align 4
  %5 = load i32, i32* %b.addr, align 4
  %sub = sub nsw i32 %4, %5
  %6 = load i32, i32* %res, align 4
  %sub2 = sub nsw i32 %6, %sub
  store i32 %sub2, i32* %res, align 4
  br label %if.end

if.end:                                           ; preds = %if.else, %if.then
  %7 = load i32, i32* %res, align 4
  ret i32 %7
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #1 {
entry:
  %a.addr.i = alloca i32, align 4
  %b.addr.i = alloca i32, align 4
  %res.i = alloca i32, align 4
  %retval = alloca i32, align 4
  store i32 0, i32* %retval, align 4
  %0 = bitcast i32* %a.addr.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* %0)
  %1 = bitcast i32* %b.addr.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* %1)
  %2 = bitcast i32* %res.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* %2)
  store i32 5, i32* %a.addr.i, align 4
  store i32 3, i32* %b.addr.i, align 4
  store i32 0, i32* %res.i, align 4
  %3 = load i32, i32* %a.addr.i, align 4
  %cmp.i = icmp slt i32 %3, 10
  br i1 %cmp.i, label %if.then.i, label %if.else.i

if.then.i:                                        ; preds = %entry
  %4 = load i32, i32* %a.addr.i, align 4
  %5 = load i32, i32* %b.addr.i, align 4
  %add.i = add nsw i32 %4, %5
  %6 = load i32, i32* %res.i, align 4
  %add1.i = add nsw i32 %6, %add.i
  store i32 %add1.i, i32* %res.i, align 4
  br label %func.exit

if.else.i:                                        ; preds = %entry
  %7 = load i32, i32* %a.addr.i, align 4
  %8 = load i32, i32* %b.addr.i, align 4
  %sub.i = sub nsw i32 %7, %8
  %9 = load i32, i32* %res.i, align 4
  %sub2.i = sub nsw i32 %9, %sub.i
  store i32 %sub2.i, i32* %res.i, align 4
  br label %func.exit

func.exit:                                        ; preds = %if.then.i, %if.else.i
  %10 = load i32, i32* %res.i, align 4
  %11 = bitcast i32* %a.addr.i to i8*
  call void @llvm.lifetime.end.p0i8(i64 4, i8* %11)
  %12 = bitcast i32* %b.addr.i to i8*
  call void @llvm.lifetime.end.p0i8(i64 4, i8* %12)
  %13 = bitcast i32* %res.i to i8*
  call void @llvm.lifetime.end.p0i8(i64 4, i8* %13)
  ret i32 %10
}

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #2

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #2

attributes #0 = { alwaysinline nounwind uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { argmemonly nofree nosync nounwind willreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"uwtable", i32 1}
!2 = !{i32 7, !"frame-pointer", i32 2}
!3 = !{!"clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)"}
