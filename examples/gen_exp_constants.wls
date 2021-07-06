#!/usr/bin/env wolframscript

Import[FileNameJoin @ {DirectoryName[$InputFileName], "..", "tools", "interval.wl"}];

print[value_] :=
    Print[intervalExpression[value] <> ","];

n = 14;

(* initial value of y *)
print[2^(-1/2) Log[2]^n / n!];
print[2^(1/2) Log[2]^n / n!];

Print["----"];

(* Taylor coefficients *)
For[i = 0, i < n, i++,
  print[Log[2]^i / i!];
 ]
