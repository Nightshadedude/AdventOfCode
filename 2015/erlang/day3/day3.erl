-module(day3).
-export([part1/1, part2/1]).

part1(FileName) -> 
    L = readlines(FileName),
    Visited = sets:new(),
    sets:size(pop_and_eval({L,Visited, 0, 0})).

part2(FileName) ->
    L = readlines(FileName),
    Visited = sets:new(),
    LE = lists:enumerate(L),
    pop_and_eval_robo({LE,Visited, 0, 0, 0, 0}).
    
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

pop_and_eval({[], Visited, X, Y}) -> 
    sets:add_element({X,Y},Visited);
    
pop_and_eval({[H|T], Visited, X, Y}) ->
    New_Visited = sets:add_element({X,Y},Visited),
    if
        % '^' - 94
        % 'v' - 118
        % '<' - 60
        % '>' - 62
        H == 94 -> pop_and_eval({T,New_Visited,X,Y+1});
        H == 118 -> pop_and_eval({T,New_Visited,X,Y-1});
        H == 60 -> pop_and_eval({T,New_Visited,X-1,Y});
        H == 62 -> pop_and_eval({T,New_Visited,X+1,Y});

        true -> 0
    end.


pop_and_eval_robo({[], Visited, X, Y, XR, YR}) -> 
    New_Visited= sets:add_element({XR,YR},sets:add_element({X,Y},Visited)),
    sets:size(New_Visited);
pop_and_eval_robo({[{Ind,Mov}|T], Visited, X, Y, XR, YR}) ->
    New_Visited= sets:add_element({XR,YR},sets:add_element({X,Y},Visited)),
    
    if 
        Ind rem 2 == 0 -> if
            Mov == 94   -> pop_and_eval_robo({T,New_Visited,X,Y+1,XR,YR});
            Mov == 118  -> pop_and_eval_robo({T,New_Visited,X,Y-1,XR,YR});
            Mov == 60   -> pop_and_eval_robo({T,New_Visited,X-1,Y,XR,YR});
            Mov == 62   -> pop_and_eval_robo({T,New_Visited,X+1,Y,XR,YR});
            true -> 0
            end;
        true -> if
            Mov == 94  -> pop_and_eval_robo({T,New_Visited,X,Y,XR,YR+1});
            Mov == 118 -> pop_and_eval_robo({T,New_Visited,X,Y,XR,YR-1});
            Mov == 60  -> pop_and_eval_robo({T,New_Visited,X,Y,XR-1,YR});
            Mov == 62  -> pop_and_eval_robo({T,New_Visited,X,Y,XR+1,YR});
            true -> 0
            end
    end.