-module(day1).
-export([part1/1, part2/1,test/0]).

test() ->
    L = ["two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen"],
    P = [parse_first(replacer_hate(I, "")) || I <- L],
    lists:sum(P).

part1(FileName) ->
    L = parse_line(readlines(FileName)),
    P = [parse_first(I) || I <- L],
    lists:sum(P).

part2(FileName) ->
    L = parse_line(readlines(FileName)),
    P = [replacer_hate(I, "") || I <- L],
    lists:sum(P).

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

parse_line(String) ->
    string:tokens(String, "\n").

% parse_first([]) ->
%     0;

parse_first([H|T]) ->
    if
        (H > 47) and (H < 58) -> parse_last(lists:reverse(T), H-48);
        true -> parse_first(T)
    end.

parse_last([],First) ->
    {Return, _} = string:to_integer(lists:concat([First, First])),
    Return;

parse_last([H|T], First) ->
    if
        (H > 47) and (H < 58) -> {Return, _} = string:to_integer(lists:concat([First, H-48]));
        true -> Return = parse_last(T, First)
    end,
    Return.

% replacer(S) ->
%     S1 = string:replace(S,"one","1",all),
%     S2 = string:replace(S1,"two","2",all),
%     S3 = string:replace(S2,"three","3",all),
%     S4 = string:replace(S3,"four","4",all),
%     S5 = string:replace(S4,"five","5",all),
%     S6 = string:replace(S5,"six","6",all),
%     S7 = string:replace(S6,"seven","7",all),
%     S8 = string:replace(S7,"eight","8",all),
%     S9 = string:replace(S8,"nine","9",all),
%     lists:flatten(S9).

replacer_hate([], Replaced) ->
    erlang:display({replaced, lists:flatten(Replaced)}),
    {Return, _} = string:to_integer(lists:concat([lists:nth(1,Replaced), lists:nth(length(Replaced),Replaced)])),
    Return;

replacer_hate(S, Replaced) ->
    [SH|ST] = S,
    Num = (SH > 47) and (SH < 58),
    One = string:slice(S, 0, 3) == "one",
    Two = string:slice(S, 0, 3) == "two",
    Three = string:slice(S, 0, 5) == "three",
    Four = string:slice(S, 0, 4) == "four",
    Five = string:slice(S, 0, 4) == "five",
    Six = string:slice(S, 0, 3) == "six",
    Seven = string:slice(S, 0, 5) == "seven",
    Eight = string:slice(S, 0, 5) == "eight",
    Nine = string:slice(S, 0, 4) == "nine",
    if
        Num -> New_Replaced = lists:flatten([Replaced|[SH-48]]);
        One -> New_Replaced = lists:flatten([Replaced|[1]]);
        Two -> New_Replaced = lists:flatten([Replaced|[2]]);
        Three -> New_Replaced = lists:flatten([Replaced|[3]]);
        Four -> New_Replaced = lists:flatten([Replaced|[4]]);
        Five -> New_Replaced = lists:flatten([Replaced|[5]]);
        Six -> New_Replaced = lists:flatten([Replaced|[6]]);
        Seven -> New_Replaced = lists:flatten([Replaced|[7]]);
        Eight -> New_Replaced = lists:flatten([Replaced|[8]]);
        Nine -> New_Replaced = lists:flatten([Replaced|[9]]);
        true -> New_Replaced = Replaced
    end,

    replacer_hate(ST, New_Replaced).
