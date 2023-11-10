-module(day1).
-export([part1/1, part2/1]).

part1(FileName) -> 
    L = readlines(FileName),
    pop_and_eval(L).

part2(FileName) ->
    L = readlines(FileName),
    accumulate_steps({L,0,0}). %{list, sum, step}
    
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
        % '(' = 40 dec
        % ')' = 41 dec
        H == 40 -> pop_and_eval(T) + 1; 
        H == 41 -> pop_and_eval(T) - 1;
        true -> 0
    end.

accumulate_steps({[], Sum, Step}) ->
    {[], Sum, Step};
accumulate_steps({_, -1, Step}) ->
     Step;
accumulate_steps({[H|T], Sum, Step}) ->
    if
        % '(' = 40 dec
        % ')' = 41 dec
        H == 40 -> accumulate_steps({T,Sum + 1, Step + 1});
        H == 41 -> accumulate_steps({T,Sum - 1, Step + 1});
        true -> accumulate_steps({T,Sum, Step + 1})
    end.