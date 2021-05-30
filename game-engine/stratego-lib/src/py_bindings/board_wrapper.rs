use crate::board::board_utils;
use crate::board::case::{self, Case, Coordinate, PyCoord, PyState, State};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::{Color, Piece, PieceType, PyColor, PyPieceType};
use crate::board::Board;
use crate::py_bindings::evaluation_function;
use crate::simulation;
use crate::simulation::evaluation::EvaluationFunction;
use crate::utils;
use pyo3::exceptions;
use pyo3::prelude::*;
use pythonize::pythonize;
use serde::{Deserialize, Serialize};

// ["09B", "04B", "-1B", "03B"],
// ["NOP", "NOP", "NOP", "NOP"],
// ["NOP", "XXX", "NOP", "NOP"],
// ["01R", "02R", "-1R", "03R"]
pub fn parse_python_cases(py_cases: Vec<Vec<PyCase>>) -> Vec<Vec<Case>> {
    let row_len = py_cases.len() - 1;
    let mut parsed_cases: Vec<Vec<Case>> = Vec::with_capacity(row_len);
    for x in 0..=row_len {
        let col_len = py_cases[x].len() - 1;
        let mut row_parsed = Vec::with_capacity(col_len);
        for y in 0..=col_len {
            let case = py_cases[x][y].clone();
            row_parsed.push(parse_python_case(case, x as i16, y as i16));
        }
        parsed_cases.push(row_parsed);
    }
    parsed_cases
}

pub fn parse_python_case(py_case: PyCase, x: i16, y: i16) -> Case {
    match py_case.as_str() {
        "NOP" => case::create_empty_case(Coordinate::new(x, y)),
        "XXX" => case::create_unreachable_case(Coordinate::new(x, y)),
        str_piece => {
            let chars: Vec<char> = str_piece.chars().collect();
            let piece_type: String = chars[0..2].iter().map(|c| c.to_string()).collect();
            let piece_type = piece_type
                .parse::<i8>()
                .unwrap_or_else(|e| panic!("Failed to parse case {}.\n {}", str_piece, e));
            let piece_type = PieceType::from(&piece_type);
            let color = if 'B' == chars[2] {
                Color::Blue
            } else {
                Color::Red
            };
            case::create_full_case(Coordinate::new(x, y), Piece::new(piece_type, color))
        }
    }
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StrategoBoardWrapper {
    board: StrategoBoard,
}

pub type PyCase = String;

impl StrategoBoardWrapper {
    pub fn new(board: StrategoBoard) -> Self {
        StrategoBoardWrapper { board }
    }

    pub fn get_board(&self) -> &StrategoBoard {
        &self.board
    }
}

#[pymethods]
impl StrategoBoardWrapper {
    #[new]
    pub fn from(cases: Vec<Vec<PyCase>>) -> Self {
        StrategoBoardWrapper {
            board: StrategoBoard::new(parse_python_cases(cases)),
        }
    }

    pub fn get_coup(&self) -> PyResult<i128> {
        Ok(self.board.get_coup())
    }
    pub fn clone_board(&self) -> Self {
        self.clone()
    }

    pub fn state(&self) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), self.board.state())?)
    }

    pub fn moving(&mut self, from: PyCoord, to: PyCoord) -> PyResult<Vec<String>> {
        match self
            .board
            .moving(Coordinate::from(from), Coordinate::from(to))
        {
            Ok(cases) => Ok(cases.iter().map(|case| case.display()).collect()),
            Err(e) => Err(exceptions::PyTypeError::new_err(format!(
                "[Error] Wrapper - failed to move: {}",
                e.message()
            ))),
        }
    }

    pub fn get_last_coup(&self) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), self.board.get_last_coup())?)
    }

    pub fn display(&self) -> PyResult<String> {
        Ok(self.board.display())
    }

    pub fn display_by_color(&self, color: PyColor) -> PyResult<String> {
        Ok(self.board.display_by_color(&color.into()))
    }

    pub fn at(&self, coordinate: PyCoord) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        let at = self.board.get_at(&Coordinate::from(coordinate));
        Ok(pythonize(gil.python(), at)?)
    }

    pub fn place(
        &mut self,
        state: PyState,
        coordinate: PyCoord,
        piece_type: PyPieceType,
        color: PyColor,
    ) -> PyResult<Py<PyAny>> {
        if let Ok(case) = self.board.place(Case::new(
            State::from(state.as_str()),
            Coordinate::from(coordinate),
            Piece::new(piece_type.into(), color.into()),
        )) {
            let gil_holder = utils::get_gild_holder()
                .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
            let gil = gil_holder.get();
            let pythonized_case = pythonize(gil.python(), &case)?;
            Ok(pythonized_case)
        } else {
            Err(exceptions::PyTypeError::new_err("Failed to place case "))
        }
    }

    pub fn get_available_moves(&self) -> PyResult<Py<PyAny>> {
        let moves = board_utils::get_availables_moves(&self.board);
        let moves: Vec<(PyCoord, PyCoord, Color, Color)> = moves
            .iter()
            .map(|(from, to, origin_color, target_color)| {
                (
                    case::from(from),
                    case::from(to),
                    *origin_color,
                    *target_color,
                )
            })
            .collect();
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves)?)
    }

    pub fn get_available_moves_by_color(&self, color: PyColor) -> PyResult<Py<PyAny>> {
        let moves = board_utils::get_availables_moves_by_color(&self.board, &color.into());
        let moves: Vec<(PyCoord, PyCoord, Color, Color)> = moves
            .iter()
            .map(|(from, to, origin_color, target_color)| {
                (
                    case::from(from),
                    case::from(to),
                    *origin_color,
                    *target_color,
                )
            })
            .collect();
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves)?)
    }

    pub fn basic_evaluation(&self) -> PyResult<PyColor> {
        if let Some(color) = evaluation_function::basic_evaluation(&self.board) {
            Ok(String::from(color.as_str()))
        } else {
            Ok(String::from("None"))
        }
    }

    pub fn material_evaluation(
        &self,
        material_value: Vec<(PyPieceType, i16)>,
    ) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        let evaluation = evaluation_function::material_evaluation(
            &self.board,
            &translate_material_values_to_rust(material_value),
        );
        eprintln!("{:?}", evaluation);
        Ok(pythonize(gil.python(), &evaluation)?)
    }

    pub fn simulate_games_material(
        &self,
        material_values: Vec<(PyPieceType, i16)>,
        stopping_criteria: Vec<i32>,
        iteration_max: i32,
        color: PyColor,
        number_of_simulations: i32,
    ) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        let eval = EvaluationFunction::Material(material_values, stopping_criteria, color.into());
        let res = simulation::simulate_multi_thread(
            self.to_owned(),
            &simulation::choose_randomly,
            &simulation::choose_randomly,
            eval,
            number_of_simulations,
            iteration_max,
        );
        Ok(pythonize(gil.python(), &res)?)
    }
}

fn translate_material_values_to_rust(
    material_value: Vec<(PyPieceType, i16)>,
) -> Vec<(PieceType, i16)> {
    material_value
        .iter()
        .map(|(rank, value)| (PieceType::from(rank), *value))
        .collect()
}
