; ModuleID = './examples/simple_call.c'
source_filename = "./examples/simple_call.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @sub_func(i32 noundef %a, i32 noundef %b) #0 {
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

; Function Attrs: alwaysinline nounwind uwtable
define dso_local i32 @func(i32 noundef %a, i32 noundef %b) #0 {
entry:
  %a.addr.i2 = alloca i32, align 4
  %b.addr.i3 = alloca i32, align 4
  %res.i4 = alloca i32, align 4
  %a.addr.i = alloca i32, align 4
  %b.addr.i = alloca i32, align 4
  %res.i = alloca i32, align 4
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
  store i32 %1, i32* %a.addr.i2, align 4
  store i32 %2, i32* %b.addr.i3, align 4
  store i32 0, i32* %res.i4, align 4
  %3 = load i32, i32* %a.addr.i2, align 4
  %cmp.i5 = icmp slt i32 %3, 10
  br i1 %cmp.i5, label %if.then.i8, label %if.else.i11

if.then.i8:                                       ; preds = %if.then
  %4 = load i32, i32* %a.addr.i2, align 4
  %5 = load i32, i32* %b.addr.i3, align 4
  %add.i6 = add nsw i32 %4, %5
  %6 = load i32, i32* %res.i4, align 4
  %add1.i7 = add nsw i32 %6, %add.i6
  store i32 %add1.i7, i32* %res.i4, align 4
  br label %sub_func.exit12

if.else.i11:                                      ; preds = %if.then
  %7 = load i32, i32* %a.addr.i2, align 4
  %8 = load i32, i32* %b.addr.i3, align 4
  %sub.i9 = sub nsw i32 %7, %8
  %9 = load i32, i32* %res.i4, align 4
  %sub2.i10 = sub nsw i32 %9, %sub.i9
  store i32 %sub2.i10, i32* %res.i4, align 4
  br label %sub_func.exit12

sub_func.exit12:                                  ; preds = %if.then.i8, %if.else.i11
  %10 = load i32, i32* %res.i4, align 4
  %11 = load i32, i32* %res, align 4
  %add = add nsw i32 %11, %10
  store i32 %add, i32* %res, align 4
  br label %if.end

if.else:                                          ; preds = %entry
  %12 = load i32, i32* %a.addr, align 4
  %13 = load i32, i32* %b.addr, align 4
  store i32 %12, i32* %a.addr.i, align 4
  store i32 %13, i32* %b.addr.i, align 4
  store i32 0, i32* %res.i, align 4
  %14 = load i32, i32* %a.addr.i, align 4
  %cmp.i = icmp slt i32 %14, 10
  br i1 %cmp.i, label %if.then.i, label %if.else.i

if.then.i:                                        ; preds = %if.else
  %15 = load i32, i32* %a.addr.i, align 4
  %16 = load i32, i32* %b.addr.i, align 4
  %add.i = add nsw i32 %15, %16
  %17 = load i32, i32* %res.i, align 4
  %add1.i = add nsw i32 %17, %add.i
  store i32 %add1.i, i32* %res.i, align 4
  br label %sub_func.exit

if.else.i:                                        ; preds = %if.else
  %18 = load i32, i32* %a.addr.i, align 4
  %19 = load i32, i32* %b.addr.i, align 4
  %sub.i = sub nsw i32 %18, %19
  %20 = load i32, i32* %res.i, align 4
  %sub2.i = sub nsw i32 %20, %sub.i
  store i32 %sub2.i, i32* %res.i, align 4
  br label %sub_func.exit

sub_func.exit:                                    ; preds = %if.then.i, %if.else.i
  %21 = load i32, i32* %res.i, align 4
  %22 = load i32, i32* %res, align 4
  %sub = sub nsw i32 %22, %21
  store i32 %sub, i32* %res, align 4
  br label %if.end

if.end:                                           ; preds = %sub_func.exit, %sub_func.exit12
  %23 = load i32, i32* %res, align 4
  ret i32 %23
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #1 {
entry:
  %a.addr.i2.i = alloca i32, align 4
  %b.addr.i3.i = alloca i32, align 4
  %res.i4.i = alloca i32, align 4
  %a.addr.i.i = alloca i32, align 4
  %b.addr.i.i = alloca i32, align 4
  %res.i.i = alloca i32, align 4
  %a.addr.i = alloca i32, align 4
  %b.addr.i = alloca i32, align 4
  %res.i = alloca i32, align 4
  %retval = alloca i32, align 4
  store i32 0, i32* %retval, align 4
  store i32 5, i32* %a.addr.i, align 4
  store i32 3, i32* %b.addr.i, align 4
  store i32 0, i32* %res.i, align 4
  %0 = load i32, i32* %a.addr.i, align 4
  %cmp.i = icmp slt i32 %0, 10
  br i1 %cmp.i, label %if.then.i, label %if.else.i

if.then.i:                                        ; preds = %entry
  %1 = load i32, i32* %a.addr.i, align 4
  %2 = load i32, i32* %b.addr.i, align 4
  store i32 %1, i32* %a.addr.i2.i, align 4
  store i32 %2, i32* %b.addr.i3.i, align 4
  store i32 0, i32* %res.i4.i, align 4
  %3 = load i32, i32* %a.addr.i2.i, align 4
  %cmp.i5.i = icmp slt i32 %3, 10
  br i1 %cmp.i5.i, label %if.then.i8.i, label %if.else.i11.i

if.then.i8.i:                                     ; preds = %if.then.i
  %4 = load i32, i32* %a.addr.i2.i, align 4
  %5 = load i32, i32* %b.addr.i3.i, align 4
  %add.i6.i = add nsw i32 %4, %5
  %6 = load i32, i32* %res.i4.i, align 4
  %add1.i7.i = add nsw i32 %6, %add.i6.i
  store i32 %add1.i7.i, i32* %res.i4.i, align 4
  br label %sub_func.exit12.i

if.else.i11.i:                                    ; preds = %if.then.i
  %7 = load i32, i32* %a.addr.i2.i, align 4
  %8 = load i32, i32* %b.addr.i3.i, align 4
  %sub.i9.i = sub nsw i32 %7, %8
  %9 = load i32, i32* %res.i4.i, align 4
  %sub2.i10.i = sub nsw i32 %9, %sub.i9.i
  store i32 %sub2.i10.i, i32* %res.i4.i, align 4
  br label %sub_func.exit12.i

sub_func.exit12.i:                                ; preds = %if.else.i11.i, %if.then.i8.i
  %10 = load i32, i32* %res.i4.i, align 4
  %11 = load i32, i32* %res.i, align 4
  %add.i = add nsw i32 %11, %10
  store i32 %add.i, i32* %res.i, align 4
  br label %func.exit

if.else.i:                                        ; preds = %entry
  %12 = load i32, i32* %a.addr.i, align 4
  %13 = load i32, i32* %b.addr.i, align 4
  store i32 %12, i32* %a.addr.i.i, align 4
  store i32 %13, i32* %b.addr.i.i, align 4
  store i32 0, i32* %res.i.i, align 4
  %14 = load i32, i32* %a.addr.i.i, align 4
  %cmp.i.i = icmp slt i32 %14, 10
  br i1 %cmp.i.i, label %if.then.i.i, label %if.else.i.i

if.then.i.i:                                      ; preds = %if.else.i
  %15 = load i32, i32* %a.addr.i.i, align 4
  %16 = load i32, i32* %b.addr.i.i, align 4
  %add.i.i = add nsw i32 %15, %16
  %17 = load i32, i32* %res.i.i, align 4
  %add1.i.i = add nsw i32 %17, %add.i.i
  store i32 %add1.i.i, i32* %res.i.i, align 4
  br label %sub_func.exit.i

if.else.i.i:                                      ; preds = %if.else.i
  %18 = load i32, i32* %a.addr.i.i, align 4
  %19 = load i32, i32* %b.addr.i.i, align 4
  %sub.i.i = sub nsw i32 %18, %19
  %20 = load i32, i32* %res.i.i, align 4
  %sub2.i.i = sub nsw i32 %20, %sub.i.i
  store i32 %sub2.i.i, i32* %res.i.i, align 4
  br label %sub_func.exit.i

sub_func.exit.i:                                  ; preds = %if.else.i.i, %if.then.i.i
  %21 = load i32, i32* %res.i.i, align 4
  %22 = load i32, i32* %res.i, align 4
  %sub.i = sub nsw i32 %22, %21
  store i32 %sub.i, i32* %res.i, align 4
  br label %func.exit

func.exit:                                        ; preds = %sub_func.exit12.i, %sub_func.exit.i
  %23 = load i32, i32* %res.i, align 4
  ret i32 %23
}

attributes #0 = { alwaysinline nounwind uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"uwtable", i32 1}
!2 = !{i32 7, !"frame-pointer", i32 2}
!3 = !{!"clang version 14.0.6 (https://github.com/llvm/llvm-project.git f28c006a5895fc0e329fe15fead81e37457cb1d1)"}
