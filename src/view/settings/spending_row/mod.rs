// MyCitadel desktop wallet: bitcoin & RGB wallet based on GTK framework.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoraprime.ch>
//
// Copyright (C) 2022 by Pandora Prime SA, Switzerland.
//
// This software is distributed without any warranty. You should have received
// a copy of the AGPL-3.0 License along with this software. If not, see
// <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

mod view_model;
mod widget;

pub use view_model::{Condition, SpendingModel};
pub use widget::RowWidgets;

pub(self) const MIN_YEAR: u32 = 2023;
pub(self) const MAX_YEAR: u32 = 2223;
