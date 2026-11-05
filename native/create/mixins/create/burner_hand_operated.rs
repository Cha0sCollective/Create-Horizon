fn create_burner_hand_operated(name: &str) -> std::ops::ControlFlow<bool> {
    if name == "create:blaze_burner" {
        std::ops::ControlFlow::Break(true)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
