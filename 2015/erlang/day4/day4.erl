-module(day4).
-export([part1/0, part2/1]).


part1() -> 
    Input = "abcdef", %example1
    iter(0, Input),
    Input1 = "pqrstuv", %example2
    iter(0, Input1),
    Input2 = "ckczppom", %input
    iter(0, Input2),
    "Complete".

part2(FileName) ->
    FileName.


md5_builder(Input, Num) ->
    Combined = io_lib:format("~s~p",[Input, Num]),
    MD5 = crypto:hash(md5, list_to_binary(Combined)),
    MD5.

md5_checker(Check_Val) ->
    Binary = <<0,0,16,0,0,0,0,0,0,0,0,0,0,0,0,0>>,
    Check_Val < Binary.

iter(10_000_000, Input) ->
    erlang:display(Input),
    erlang:display("Tail reached, exiting");
iter(Val, Input) ->
    Res = md5_checker(md5_builder(Input, Val)),
    if
        Res -> 
            erlang:display(Input),
            erlang:display(Val);
        true -> iter(Val+1, Input)
    end.

