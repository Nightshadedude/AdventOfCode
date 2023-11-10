-module(day1).
-export([part1/1]).

part1(FileName) -> 
    L = readlines(FileName),
    pop_and_eval(L).
    
readlines(FileName) ->
    {ok, Device} = file:open(FileName, [read]),
    try get_all_lines(Device)
      after file:close(Device)
    end.

get_all_lines(Device) ->
    case io:get_line(Device, "") of
        eof  -> [];
        Line -> Line ++ get_all_lines(Device)
    end.


pop_and_eval([]) -> 0;
pop_and_eval([H|T]) ->
        if
        H == 40 -> pop_and_eval(T) + 1;
        H == 41 -> pop_and_eval(T) + -1;
        true -> 0
    end.