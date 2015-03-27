use super::geomorph::add_cache_chunk;
use super::Biome::{Overland, Dungeon};

// Geomorph data.

/*
1######B222 Template for herringbone prefabs
1##########
1########## Cells at positions A, B and C must have an open tile.
A########## On each half, the openings A, B and C must be connected.
########### The two halves may or may not be connected.
########### This ensures automatic map connectivity, while not
########### making the map trivially open.
##########B
##########2 The numbered lines are parameters by which the openings
##########2 are positioned. When changing the position of an opening
##########2 for an alternative set, lines with the same symbol must
3*********1 remain at equal length.
3*********1
3*********1
3*********A
3**********
C**********
***********
***********
***********
***********
33333C*****
*/

/// Initialize global geomorph cache.
pub fn init_geomorphs() {
    add_cache_chunk(Overland, 0, "\
%%%%%%%,%%%
%%%%%%%,%%%
%%%%%%%,%%%
,,,%%%,,%%%
%%,,,,,%%%%
%%,%,,,%%%%
%%%,,%,%%%%
%%%%%%%,,,,
%%%%%%%,%%%
%%%%%%%%%%%
%%%%%%%%%%%
%%%%%%%%%%%
%%%%%%%%%%%
%%%%%%%,%%%
%%%%%%,,,,,
%%%%%,,%%%%
,,,,%,,%%%%
%%%,,,%%%%%
%%%,,%%%%%%
%%%%,%%%%%%
%%%%,%%%%%%
%%%%%,%%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%%%,%%%
%%%%%%,,%%%
%%%%,,,%%%%
,,,%%,,%%%%
%%,,%%,,%%%
%%%%,,,%%%%
%%%%,,%%%%%
%%%%,,%%,,,
%%%%%,,,,%%
%%%%%,,%%%%
%%%,,,%%%%%
%%%%,,%%%%%
%%%%,,%%%%%
%%%%,%%%%%%
%%%%,,%,,,,
%%%%%%,,%%%
,,%%%,,%%%%
%%,,,,%%%%%
%%%%,,%%%%%
%%%%%,,%%%%
%%%%%,,%%%%
%%%%%,%%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%,,,%%%
%%,,,,,,,%%
%,,,%,,,,,%
,,,,,,%,,%%
,,,,,,,,,,%
%,,,,,,,,,%
%,,,%,,%,,,
%%o,,,,,,,,
%%,,,,,%,,,
%,,,,%,,,,%
%%,,,,,,,%%
%%,%,,,o,,%
%%,,,,,,,%%
%%%,,,,o%%%
%%%%%,,,,,,
%%%%%%%%%,%
,,,,%%%%,,%
%%%,%%%%,%%
%%%%,%%,,%%
%%%%%,,,%%%
%%%%%,%%%%%
%%%%%,%%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%,,,%%%
%%%,,,,,,,%
%%,,,,,,,,%
,,,,%,%,,,%
,,,,,,,,,,%
,,,%,,,,,,%
%,,,o,,,,,,
%,,,,%,,,,,
%,%,,,,%,,%
%,,,,,o,,,%
%%,%,,,,%%%
%,,,,,,,o%%
%%o,,,,,,,%
%%,,,,,,,,,
%,,,%,,,,,,
%,,,,,,,,,%
,,,%,,,%,,%
,,,,,,,,,%%
%,,,,,,,,,%
%%o%,,,,,%%
%%%,,,,,%%%
%%%%,,,%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%%%,%%%
%%%,,,,,%%%
%%,,%%%,%%%
,,,%%%%,,%%
%%,,%%,,,%%
%%%,,,,%%%%
%%,,,,,,,,%
%%,,,,,%%%,
%%%%%%%%%%%
%%%%%%%%%%%
%%%%%,,,%%%
%%%,,,,,%%%
%%%,,,,,,,%
%%,,..,,,,%
%%,A./..,,,
%,,..//.,,%
,,,.//.A,%%
%%,,./.,,%%
%,,,A,.,,%%
%%,,,,,,,%%
%%%,,,,,%%%
%%%%%,%%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%%%,%%%
%%%%%,,,%%%
%%,,,,,,%%%
,,,,,,,%,,%
%%,,,,,%%,%
%/%,,,,,%%%
%%%%/%%,/%%
%%%%%%%,,,,
%%%%%%%,/%%
%%%%%,,,%%%
%///..//%%%
%/.A..../%%
%.A..A..,%%
%./.a../%,%
%%.A..A/%,,
%%...A,,%%%
,,,,,//%/%%
%%%,,%%%%%%
%%%%,%%/%%%
%%%%%,%%%%%
%%%%%,,%%%%
%%%%%,,%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%%%,%%%
%%%%%%,,,%%
%%%%,,,,,%%
,,,,,,,,,%%
%%,,%%,,,%%
%%%%%,,,%%%
%%%%%%,,%,%
%%%%,,,,,,,
%%%%,%%,,,%
%%%%%%%,,%%
%%%%%%%,,%%
%%%%%,,,%%%
%%%%%,%%%%%
%%%%%,%%%%%
%%%%%,,,,,,
%%,,,,,,,,%
,,,,,,,%%%%
%%%,,,%%%%%
%%%%,,,%%%%
%%%%%,,,%%%
%%%%%,,%%%%
%%%%%,,%%%%");

    add_cache_chunk(Overland, 0, "\
%%%%%%,,%%%
%%%%%%,,,%%
%%%%,,,,,%%
,,,,,,,,,%%
%%,,%%,,,%%
%%%%%,,,%%%
%%%%%%,,%,%
%%%o,,,,,,,
%%%~~%%,,,%
==%~~~o,,~%
=======~~~=
===========
~====~~~===
,,,~~~%%%%%
,%%%%,,,,,,
%%,,,,,,,,%
,,,,,,,%%%%
%%%,,,%%%%%
%%%%,,,%%%%
%%%%%,,,%%%
%%%%%,,%%%%
%%%%%,,%%%%");

    add_cache_chunk(Overland, 0, "\
,,,,,,,,,,,
,========,,
,=#|#=#|#=,
,=|.###.|=,
,=#.+.+.#=,
,=###+###=,
,==#b.b#==,
,==#>.b#==,
,==|...|==,
,==##+##==,
,..+...+..,
,..+...+..,
,==##+##==,
,==|...|==,
,==#.T.#==,
,==#...#==,
,=###+###=,
,=#.+.+.#=,
,=|.###.|=,
,=#|#=#|#=,
,,========,
,,,,,,,,,,,");

    add_cache_chunk(Overland, 0, "\
,,,,,,,,,,,
,,,,,,,,,,,
,,******,,,
,****..**,,
,*.*.....,,
,*.**!..*,,
,*.**..**,,
,*....***,,
,**I.III**,
,,**..****,
,**....!**,
,**.*.***,,
,***.**X*,,
,*...XXX*,,
,**...X**,,
,*##+#XX*,,
,*#....X**,
,*#..>X#**,
,**!..X#**,
,,**###***,
,,*******,,
,,,,,,,,,,,");

    add_cache_chunk(Dungeon, 0, "\
#######.###
#######+###
##......###
.+......###
##......###
##g.....###
##......###
##g.....+..
##......###
###+#######
###.#######
###.#######
###.#######
###.#######
###........
#####.#####
......#####
#####.#####
#####.#####
#####.#####
#####.#####
#####.#####");

    add_cache_chunk(Dungeon, 0, "\
#######.###
#######.###
#######.###
........###
#######.###
#######.###
#######.###
#######....
###########
###########
###########
###########
###########
###########
#####......
#####.#####
......#####
#####.#####
#####.#####
#####.#####
#####.#####
#####.#####");

    add_cache_chunk(Dungeon, 0, "\
#######.###
#######.###
#######.###
........###
##.####.###
##.####.###
##.####.###
##.........
##.##.#####
##.#q#..#q#
##.#qq#..##
##.#qqq#..#
#q#.#qqq#.#
#qq#.####.#
#qqq#......
#####.#####
......#####
#####.#####
#####.#####
#####.#####
#####.#####
#####.#####");
    add_cache_chunk(Dungeon, 0, "\
#######.###
#######+###
##.......##
.+...#...##
##..###..##
##...#...##
##.#...#.##
##.......+.
##.......##
##.#...#.##
##.......##
##.......##
##.#...#.##
##.......##
##.......+.
##.#...#.##
.+...#...##
##..###..##
##...#...##
##.......##
#####+#####
#####.#####");

    add_cache_chunk(Dungeon, 0, "\
*******.***
****#...***
#####...#**
.......##**
##.....+..*
*..##..#..*
**..#++##**
**#|#......
**....!..**
**..!.....*
**..XX..!.*
**!.XXX..**
**...XX..**
**X......**
**XXXXX....
**.XXX...**
.......X.**
**..!..XX**
***....XX**
****..**XX*
*****.*XXX*
*****.*****");

    add_cache_chunk(Dungeon, 0, "\
#######.###
#######.###
#######.###
........###
#.......###
#.......###
#.......###
#..........
###=====###
###=====###
###=====###
###=====###
###IIIII###
###.....###
###........
###.....###
........###
###.....###
###.....###
###.....###
#####.#####
#####.#####");

    add_cache_chunk(Dungeon, 0, "\
#######.###
#.........#
#.........#
..........#
#.........#
#.........#
#.........#
#..........
#..#.#....#
#.........#
#..#.>.#..#
#.........#
#....#.#..#
#.........#
#..........
#.........#
..........#
#.........#
#.........#
#.........#
#.........#
#####.#####");
}