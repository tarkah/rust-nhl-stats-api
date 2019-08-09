use std::fmt;

pub use crate::models::{EnumStandingTypes, EnumStatTypes, EnumExpandSchedule, EnumExpandTeams};

impl fmt::Display for EnumStatTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnumStatTypes::ByDayOfWeek => "byDayOfWeek",
            EnumStatTypes::ByMonth => "byMonth",
            EnumStatTypes::GoalsByGameSituation => "goalsByGameSituation",
            EnumStatTypes::HomeAndAway => "homeAndAway",
            EnumStatTypes::OnPaceRegularSeason => "onPaceRegularSeason",
            EnumStatTypes::RegularSeasonStatRankings => "regularSeasonStatRankings",
            EnumStatTypes::StatsSingleSeason => "statsSingleSeason",
            EnumStatTypes::VsConference => "vsConference",
            EnumStatTypes::VsDivision => "vsDivision",
            EnumStatTypes::VsTeam => "vsTeam",
            EnumStatTypes::WinLoss => "winLoss",
            EnumStatTypes::ByDayOfWeekPlayoffs => "byDayOfWeekPlayoffs",
            EnumStatTypes::ByMonthPlayoffs => "byMonthPlayoffs",
            EnumStatTypes::CareerPlayoffs => "careerPlayoffs",
            EnumStatTypes::CareerRegularSeason => "careerRegularSeason",
            EnumStatTypes::GameLog => "gameLog",
            EnumStatTypes::GoalsByGameSituationPlayoffs => "goalsByGameSituationPlayoffs",
            EnumStatTypes::HomeAndAwayPlayoffs => "homeAndAwayPlayoffs",
            EnumStatTypes::PlayoffGameLog => "playoffGameLog",
            EnumStatTypes::PlayoffStatRankings => "playoffStatRankings",
            EnumStatTypes::StatsSingleSeasonPlayoffs => "statsSingleSeasonPlayoffs",
            EnumStatTypes::VsConferencePlayoffs => "vsConferencePlayoffs",
            EnumStatTypes::VsDivisionPlayoffs => "vsDivisionPlayoffs",
            EnumStatTypes::VsTeamPlayoffs => "vsTeamPlayoffs",
            EnumStatTypes::WinLossPlayoffs => "winLossPlayoffs",
            EnumStatTypes::YearByYear => "yearByYear",
            EnumStatTypes::YearByYearPlayoffs => "yearByYearPlayoffs",
            EnumStatTypes::YearByYearPlayoffsRank => "yearByYearPlayoffsRank",
            EnumStatTypes::YearByYearRank => "yearByYearRank",
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for EnumStandingTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnumStandingTypes::ByConference => "byConference",
            EnumStandingTypes::ByDivision => "byDivision",
            EnumStandingTypes::ByLeague => "byLeague",
            EnumStandingTypes::DivisionLeaders => "divisionLeaders",
            EnumStandingTypes::Postseason => "postseason",
            EnumStandingTypes::Preseason => "preseason",
            EnumStandingTypes::RegularSeason => "regularSeason",
            EnumStandingTypes::WildCard => "wildCard",
            EnumStandingTypes::WildCardWithLeaders => "wildCardWithLeaders",
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for EnumExpandSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnumExpandSchedule::Broadcasts => "schedule.broadcasts",
            EnumExpandSchedule::Linescore=> "schedule.linescore",
            EnumExpandSchedule::Ticket=> "schedule.ticket",            
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for EnumExpandTeams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnumExpandTeams::TeamRoster => "team.roster",
            EnumExpandTeams::PersonNames => "person.names",
            EnumExpandTeams::TeamScheduleNext => "team.schedule.next",
            EnumExpandTeams::TeamSchedulePrevious => "team.schedule.previous",
            EnumExpandTeams::TeamStats => "team.stats",
        };

        write!(f, "{}", s)
    }
}