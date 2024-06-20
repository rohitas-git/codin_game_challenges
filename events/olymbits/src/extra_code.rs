
struct PlayerScore {
    final_score: u32,
    nb_gold_medals: u32,
    nb_silver_medals: u32,
    nb_bronze_medals: u32,
}
impl PlayerScore {
    fn calculate_final_score(&mut self) {
        self.final_score = self.nb_silver_medals + self.nb_gold_medals * 3;
    }

    fn increase_gold_medal(&mut self) {
        self.nb_gold_medals += 1;
        self.calculate_final_score();
    }

    fn increase_silver_medal(&mut self) {
        self.nb_silver_medals += 1;
        self.calculate_final_score();
    }

    fn increase_bronze_medal(&mut self) {
        self.nb_bronze_medals += 1;
    }

    fn final_score(&self) -> u32 {
        self.final_score
    }

    fn new(
        final_score: u32,
        nb_gold_medals: u32,
        nb_silver_medals: u32,
        nb_bronze_medals: u32,
    ) -> Self {
        PlayerScore {
            final_score,
            nb_gold_medals,
            nb_silver_medals,
            nb_bronze_medals,
        }
    }
}