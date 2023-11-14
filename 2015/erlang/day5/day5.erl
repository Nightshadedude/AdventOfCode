-module(day5).
-export([part1/1, part2/1]).

part1(FileName) -> 
    L = parse(readlines(FileName)),
    erlang:display(length(L)),
    CL = [S || S <- L, count_vowel(S) > 2],
    erlang:display(length(CL)),
    DL = [S || S <- CL, count_dupe(S) > 0],
    erlang:display(length(DL)),
    BL = [S || S <- DL, count_bad_string(S) == 0],
    erlang:display(length(BL)),
    BL
    .

part2(FileName) ->
    L = parse(readlines(FileName)),
    erlang:display(length(L)),
    PL = [S || S <- L, count_pair(S, [])],
    erlang:display(length(PL)),
    RL = [S || S <- PL, count_repeat(S)],
    erlang:display(length(RL)).
    
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

parse(String) ->
    string:tokens(String, "\n").

count_vowel(String) ->
    length([C || C <- String, lists:member(C, [$a,$e,$i,$o,$u])]).

count_dupe(String) ->
    SS = ["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz"],
    length([C || C <- SS, string:find(String, C) =/= nomatch]).

count_bad_string(String) ->
    SS = ["ab", "cd", "pq", "xy"],
    length([C || C <- SS, string:find(String, C) =/= nomatch]).

count_pair([H|[H1|T]], Pairs) ->
    Pair = [H,H1],
    Found = string:find(T, Pair) =/= nomatch,
    if Found =:= true -> true;
        true -> 
            New_String = lists:flatten([H1,T]),
            count_pair(New_String, [Pairs,{Pair, Found}])
    end;

count_pair(_, _) ->
    false.

count_repeat([H|[H1|[H2|T]]]) ->
    Match = H =:= H2,
    if Match -> true;
        true -> 
            New_String = lists:flatten([H1,H2,T]),
            count_repeat(New_String)
    end;

count_repeat(_) ->
    false.


