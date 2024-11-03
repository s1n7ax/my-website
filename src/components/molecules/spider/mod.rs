use std::cell::RefCell;
use std::f64::consts::PI;
use std::ops::Deref as _;
use std::rc::Rc;

use ev::MouseEvent;
use leptos::*;
use leptos_use::use_element_size;
use leptos_use::UseElementSizeReturn;
use logging::log;
use rand::Rng;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

stylance::import_style!(styles, "spider.module.scss");

#[derive(Copy, Clone)]
struct Point<'a> {
	x: i32,
	y: i32,
	radius: i32,
	color: &'a str,
}

#[component]
pub fn Spider() -> impl IntoView {
	let canvas_ref = create_node_ref::<html::Canvas>();
	let color_palette = ["#DC8665", "#138086", "#534666", "#CD7672", "#EEB462"];
	let points: Rc<RefCell<Vec<Point>>> = Rc::new(RefCell::new(vec![]));
	let UseElementSizeReturn { width, height } = use_element_size(canvas_ref);

	let get_canvas_and_context =
		move || -> Option<(html::HtmlElement<html::Canvas>, CanvasRenderingContext2d)> {
			if let Some(canvas) = canvas_ref.get() {
				let html_canvas = canvas.deref();
				if let Ok(Some(ctx)) = html_canvas.get_context("2d") {
					if let Ok(ctx) = ctx.dyn_into::<CanvasRenderingContext2d>() {
						return Some((canvas, ctx));
					}
				}
			}
			None
		};

	let clear_canvas = move |ctx: CanvasRenderingContext2d, width: u32, height: u32| {
		ctx.clear_rect(0_f64, 0_f64, width as f64, height as f64);
	};

	let draw_circles = move |points: Vec<Point>, ctx: CanvasRenderingContext2d| {
		points.iter().for_each(|point| {
			ctx.set_fill_style(&JsValue::from_str(point.color));
			ctx.set_global_alpha(0.5);
			ctx.begin_path();
			let _ = ctx.arc(
				point.x.into(),
				point.y.into(),
				point.radius.into(),
				0_f64,
				PI * 2_f64,
			);
			ctx.close_path();
			ctx.fill();
		});
	};

	let draw_lines = move |points_copy: Vec<Point>,
	                       ctx: CanvasRenderingContext2d,
	                       mouse_x: i32,
	                       mouse_y: i32| {
		points_copy.iter().for_each(|point| {
			let distance = ((square(mouse_x - point.x) + square(mouse_y - point.y)) as f64)
				.sqrt()
				.floor();

			if distance < 300_f64 {
				ctx.set_stroke_style(&JsValue::from_str(point.color));
				ctx.set_line_width(1_f64);
				ctx.set_global_alpha((301_f64 - distance) / 800_f64);
				ctx.begin_path();
				ctx.move_to(point.x as f64, point.y as f64);
				ctx.line_to(mouse_x as f64, mouse_y as f64);
				ctx.close_path();
				ctx.stroke();
			};
		});
	};

	let update_points = {
		let points_ref = Rc::clone(&points);
		move |count: i32, width: u32, height: u32| {
			let mut rg = rand::thread_rng();

			(0..count)
				.map(move |_| Point {
					x: rg.gen_range(0..=width) as i32,
					y: rg.gen_range(0..=height) as i32,
					radius: rg.gen_range(7..10),
					color: color_palette[rg.gen_range(0..=4)],
				})
				.for_each(|point| {
					points_ref.borrow_mut().push(point);
				})
		}
	};

	create_effect({
		let points_ref = Rc::clone(&points);
		move |_| {
			if let Some((canvas, ctx)) = get_canvas_and_context() {
				let xw = canvas.offset_width() as u32;
				let xh = canvas.offset_height() as u32;
				let w = width.get() as u32;
				let h = height.get() as u32;
				log!("ref w:{} h:{}   use w:{} h:{}", xw, xh, w, h);

				canvas.set_width(w);
				canvas.set_height(h);
				update_points(50, w, h);

				log!("clear canvas");
				clear_canvas(ctx.clone(), w, h);
				draw_circles(points_ref.borrow().clone(), ctx.clone());
			};
		}
	});

	let redraw_canvas = {
		let points_clone = Rc::clone(&points);
		move |ev: MouseEvent| {
			if let Some((canvas, ctx)) = get_canvas_and_context() {
				let points_copy = points_clone.borrow().clone();

				clear_canvas(ctx.clone());
				draw_circles(points_copy.clone(), ctx.clone());

				let bound_rect = canvas.get_bounding_client_rect();
				let cursor_x = ev.x() - bound_rect.left() as i32;
				let cursor_y = ev.y() - bound_rect.top() as i32;

				draw_lines(points_copy, ctx, cursor_x, cursor_y);
			}
		}
	};

	let clear_lines = {
		let points_ref = Rc::clone(&points);
		move |_| {
			if let Some((canvas, ctx)) = get_canvas_and_context() {
				let points = points_ref.borrow().clone();
				clear_canvas(canvas, ctx.clone());
				draw_circles(points, ctx);
			}
		}
	};

	view! {
	<canvas
		node_ref=canvas_ref
		on:mousemove=redraw_canvas
		on:mouseout=clear_lines
		class=styles::canvas
	/>
	}
}

fn square(x: i32) -> i32 {
	x * x
}
