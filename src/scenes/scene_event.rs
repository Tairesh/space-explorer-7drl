use super::Scene;

pub enum SceneEvent {
    ChangeScene(Scene),
    Custom(u8),
    Exit,
}
