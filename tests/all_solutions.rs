use std::collections::HashMap;

use aoc::*;

#[test]
fn all_solutions() -> Result<()> {
    let mut expected = HashMap::<(u32, u32), _>::new();
    expected.insert((2021, 1), ["1393", "1359"]);
    expected.insert((2021, 2), ["2027977", "1903644897"]);
    expected.insert((2021, 3), ["1092896", "4672151"]);
    expected.insert((2021, 4), ["25410", "2730"]);

    expected.insert((2022, 1), ["69883", "207576"]);
    expected.insert((2022, 2), ["11906", "11186"]);
    expected.insert((2022, 3), ["8109", "2738"]);
    expected.insert((2022, 4), ["547", "843"]);
    expected.insert((2022, 5), ["TQRFCBSJJ", "RMHFJNVFP"]);
    expected.insert((2022, 6), ["1794", "2851"]);
    expected.insert((2022, 7), ["919137", "2877389"]);
    expected.insert((2022, 8), ["1560", "252000"]);
    expected.insert((2022, 9), ["6339", "2541"]);
    expected.insert(
        (2022, 10),
        [
            "13520",
            "
###...##..###..#..#.###..####..##..###..
#..#.#..#.#..#.#..#.#..#.#....#..#.#..#.
#..#.#....#..#.####.###..###..#..#.###..
###..#.##.###..#..#.#..#.#....####.#..#.
#....#..#.#....#..#.#..#.#....#..#.#..#.
#.....###.#....#..#.###..####.#..#.###..",
        ],
    );
    expected.insert((2022, 11), ["66124", "19309892877"]);
    expected.insert((2022, 12), ["330", "321"]);
    expected.insert((2022, 13), ["5852", "24190"]);

    let sols = dynamic::solutions();
    assert_eq!(sols.len(), expected.len());

    for sol in sols {
        let mut puzzle = fetch_puzzle(sol.year, sol.day)?;
        (sol.solve)(&mut puzzle)?;
        let expected = expected[&(sol.year, sol.day)];
        for (i, out) in puzzle.outputs().enumerate() {
            assert_eq!(out, expected[i], "in {}d{}p{}", sol.year, sol.day, i + 1);
        }
    }

    Ok(())
}
