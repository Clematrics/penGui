const not_clicked = "Not clicked";
const clicked = "Clicked";

fn main() {
	let perspective = ...; // Perspective
	let view_matrix = ...; // View matrix

	let lui = pengui::UserInterface::new(backend);

	loop {
		let ui = lui.new_frame(); // could be enclosed into lambda

		ui.global_transform(perspective * view_matrix);
		ui.default_theme(pengui::Theme {
			base_color: pengui::Color::new(0.0, 0.0, 0.8, 1.0),
			outline_color: pengui::Color::new(0.0, 0.0, 0.2, 1.0),
		});
		ui.scaling(1.);

		let mut text = &not_clicked;

		pengui::Window::new("Window 1")
			.size(400., 600.) // size in pixel or size relative to screen to account for dpi & stuff? Need to be careful to the ratio then. Idea : 1. = size of smaller side of screen
			.transform(...) // model matrix for the window
			.content(|&ui| {
				pengui::Column::new(|&ui| {
					if let pengui::Clicked = pengui::Button(text).build(&ui) {
						text = &clicked;
					}
				}).build(&ui);
			}).build(&ui);

		lui = ui.end_frame();
		lui.generate_layout();
		lui.draw();

		lui.register_event();
	}
}

// -------------------------------------------------------------------------------
// Interface ---------------------------------------------------------------------
// -------------------------------------------------------------------------------

struct WidgetId(ComponentId);

struct WindowDraft {
	name: &str,
	transform: matrix,
	content: std::HashMap<WidgetId, Widget>,
	fct: Fn() -> ()
}

struct Window {
	name: &str,
	transform: matrix,
	content: hashmap,
}

impl Window {
	fn new(text: &str) -> WindowDraft {
		Self {
			name: text,
			transform: identity,
			content: hashmap::new();
			fct: |&ui| { self }
		}
	}
}

impl WindowDraft {
	fn content(&mut self, f: Fn() -> ()) -> Self {
		self.fct = f
	}
}

// for build fonction. Or should be in WindowDraft?
// If in WindowDraft, easier to right & read
// Warning to types & if it can be read or not
impl WidgetDraft for WindowDraft {
	type BuildFeedback = ();
	type AchievedType = Window; // Could enforce in a more proper way the fact that widgets are all drafts until in the tree

	// Here with option 3, user interface should rather be called InterfaceTree or similar
	// For style locality without duplication, an interface tree would have rules to return
	// a changed value rather than the main ui one only if it is set, or something like this
	// However, this can apply only on a whole container scope, and not on some group of specific
	// widgets
	fn build(self, ui: &UserInterface) -> Self::BuildFeedback {
		// 1.
		// Not very flexible, each widget must have a function to add a list of widgets
		// Not consistent when a widget only has a single field ?
		ui.add_and_branch(self); // add self as a widget to the current content and move cursor to self
		self.fct(ui); // apply function of content to load content into
		ui.end_current_container(); // cursor goes up

		// 2.
		// just same method as above, but limited wrapper to avoid not closing the add_and_branch by end_current_...
		ui.add_container(self, self.fct);

		// 3.
		// Will call get_child_from_id on the root of ui (in the case of the window, the root of the final ui)
		// To retrieve the old state of the window if it existed during last frame, and thus all its previous content
		// Comparison is done by type (using std::any::Any and std::any::TypeId) and then by Id
		// If existed previous frame, return same type as self as root of UserInterface
		// Return ui or widget ?
		// Advantage with ui is custom styling with scope, but adds a wrapper around each widget
		// If widget, no wrapper but unhomogeneous API
		match ui.borrow_widget<Self::AchievedType>(id) {
			Some(restricted_ui) => {
				// compare with restricted ui, with old window as the root
				// 1. If restricted_ui is InterfaceTree,
				// could use some kind of structure restriction during assignment to limit assignments to relevant fields, maybe struct update syntax ?
				restricted_ui.widget.name = self.name;
				restricted_ui.widget.transform = self.transform;
				restricted_ui.widget.content = self.content;
				// 2. If restricted_ui is widget directly, could use some kind of structure restriction during assignment to limit assignments to relevant fields
				restricted_ui.widget = self;

				// Common
				self.fct(restricted_ui);
				ui.give_widget(restricted_ui);
				// return changed restricted_ui to ui. With id & type ?
			}
			None => {
				let restricted_ui = pengui::InterfaceTree::new(self);
				ui.give_widget(restricted_ui);
				// return fresh ui With id & type ?
				// Could be factorized with the other case: the match returns an InterfaceTree & ui.give_widget is called once on this result
			}
		}
	}
}

impl Widget for Window {
	fn get_child_from_id<T>(id: WidgetId) -> Some(InterfaceTree) { // Or rather Widget ?

	}

	fn apply_transformations(&mut self) -> Transform { // Return computed transformation
	}

	fn get_transformation(&self) -> Transformation { // Return transformation
	}

	fn capture_event() -> EventResponse { // Capture event. Is it the responsablity to the widget to propagate the event ?
		// Should not I think, but otherwise, give additional flexibility
		// EventResponse is a new wrapped event with if it can propagate to children or not, (propagate from start again? unsafe for infinite loops)
	}
}

impl Iter for Window {
	// To iter on child and draw each
}

impl IterMut for Window {
	// To iter on child and apply local transformations
}