-module(day4).
-export([part1/0, part2/0]).


part1() -> 
    Input = "abcdef", %example1
    iter(0, Input),
    Input1 = "pqrstuv", %example2
    iter(0, Input1),
    Input2 = "ckczppom", %input
    iter(0, Input2),
    "Complete".

part2() ->
    Input2 = "ckczppom", %input
    iter_2(0, Input2),
    "Complete".

md5_builder(Input, Num) ->
    Combined = io_lib:format("~s~p",[Input, Num]),
    MD5 = crypto:hash(md5, list_to_binary(Combined)),
    MD5.

md5_checker(Check_Val) ->
    Binary = <<0,0,16,0,0,0,0,0,0,0,0,0,0,0,0,0>>,
    Check_Val < Binary.

md5_checker_2(Check_Val) ->
    Binary = <<0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0>>,
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

iter_2(10_000_000, Input) ->
    erlang:display(Input),
    erlang:display("Tail reached, exiting");
iter_2(Val, Input) ->
    Res = md5_checker_2(md5_builder(Input, Val)),
    if
        Res -> 
            erlang:display(Input),
            erlang:display(Val);
        true -> iter_2(Val+1, Input)
    end.


