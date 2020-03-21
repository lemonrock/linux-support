/// Rule.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct Rule
{
	/// Action to take.
	#[serde(default)] pub action_to_take: Action,

	/// Higher values have higher priority when filtering.
	#[serde(default)] pub priority: u8,

	/// Comparisons.
	#[serde(default)] pub comparisons: Vec<Comparison>,
}
