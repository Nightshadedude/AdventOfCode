-module(day2).
-export([part1/1, part2/1]).

part1(FileName) -> 
    L = readlines(FileName),
    calculate_area({L, 0}).

part2(FileName) ->
    L = readlines(FileName),
    calculate_ribbon({L,0}).
    
readlines(FileName) ->
    {ok, Binary} = file:read_file(FileName),
    [binary_to_list(Bin) || Bin <- binary:split(Binary,<<"\n">>,[global])].

calculate_area({[], Acc}) -> Acc;
calculate_area({[H|T], Acc}) ->
    [L_S,W_S,Ht_S] = string:tokens(H, "x"),
    {L, _} = string:to_integer(L_S),
    {W, _} = string:to_integer(W_S),
    {Ht, _} = string:to_integer(Ht_S),
    Area = 2*L*W + 2*W*Ht + 2*L*Ht,
    Slack = lists:min([L*W,W*Ht,L*Ht]),
    calculate_area({T, Acc + Area + Slack}).

calculate_ribbon({[], Acc}) -> Acc;
calculate_ribbon({[H|T], Acc}) ->
    [L_S,W_S,Ht_S] = string:tokens(H, "x"),
    {L, _} = string:to_integer(L_S),
    {W, _} = string:to_integer(W_S),
    {Ht, _} = string:to_integer(Ht_S),
    Sorted = lists:sort([L,W,Ht]),
    Low = lists:nth(1,Sorted),
    Mid = lists:nth(2,Sorted),
    Ribbon = Low + Low + Mid + Mid,
    Bow = L*W*Ht,
    calculate_ribbon({T, Acc + Ribbon + Bow}).