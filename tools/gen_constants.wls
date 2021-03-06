#!/usr/bin/env wolframscript

(* Format the source code in Wolfram Workbench before committing. *)

toInterval[x_] :=
    Module[ {ulp, inf, sup},
        ulp = 2^(Floor[Log2[x]] - 52);
        inf = Floor[x, ulp];
        sup = Ceiling[x, ulp];
        {inf, sup}
    ];

toLiteral[x_] :=
    ExportString[x, "JSON"];

def[name_, value_] :=
    Module[ {inf, sup},
        {inf, sup} = toLiteral /@ toInterval[value];
        Print["pub const " <> name <> ": Self = const_interval!(" <> inf <> "," <> sup <> ");"];
    ];

def["E", E];
def["FRAC_1_PI", 1/Pi];
def["FRAC_1_SQRT_2", 1/Sqrt[2]];
def["FRAC_2_PI", 2/Pi];
def["FRAC_2_SQRT_PI", 2/Sqrt[Pi]];
def["FRAC_PI_2", Pi/2];
def["FRAC_PI_3", Pi/3];
def["FRAC_PI_4", Pi/4];
def["FRAC_PI_6", Pi/6];
def["FRAC_PI_8", Pi/8];
def["LN_10", Log[10]];
def["LN_2", Log[2]];
def["LOG10_2", Log10[2]];
def["LOG10_E", Log10[E]];
def["LOG2_10", Log2[10]];
def["LOG2_E", Log2[E]];
def["PI", Pi];
def["SQRT_2", Sqrt[2]];
def["TAU", 2*Pi];
