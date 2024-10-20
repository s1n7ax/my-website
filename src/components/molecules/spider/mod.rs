use std::cell::RefCell;
use std::f64::consts::PI;
use std::ops::Deref as _;
use std::rc::Rc;

use ev::MouseEvent;
use leptos::*;
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
	let color_palette = ["#DC8665", "#138086", "#534666", "#CD7672", "EEB462"];
	let points: Rc<RefCell<Vec<Point>>> = Rc::new(RefCell::new(vec![]));

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

	let clear_lines = move |canvas: html::HtmlElement<html::Canvas>,
	                        ctx: CanvasRenderingContext2d| {
		ctx.clear_rect(0_f64, 0_f64, canvas.width() as f64, canvas.height() as f64);
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
			ctx.fill();
		})
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
				ctx.set_global_alpha(0.5);
				ctx.begin_path();
				ctx.move_to(point.x as f64, point.y as f64);
				ctx.line_to(mouse_x as f64, mouse_y as f64);
				ctx.stroke();
			};
		});
	};

	create_effect({
		let points_clone = Rc::clone(&points);
		move |_| {
			if let Some((canvas, ctx)) = get_canvas_and_context() {
				let width = canvas.offset_width() as u32;
				let height = canvas.offset_height() as u32;

				canvas.set_width(width);
				canvas.set_height(height);

				let mut rg = rand::thread_rng();

				(0..50)
					.map(move |_| Point {
						x: rg.gen_range(0..=width) as i32,
						y: rg.gen_range(0..=height) as i32,
						radius: rg.gen_range(7..10),
						color: color_palette[rg.gen_range(0..=4)],
					})
					.for_each(|point| {
						points_clone.borrow_mut().push(point);
					});

				draw_circles(points_clone.borrow().clone(), ctx);
			};
		}
	});

	let on_mouse_move = {
		let points_clone = Rc::clone(&points);
		move |ev: MouseEvent| {
			if let Some((canvas, ctx)) = get_canvas_and_context() {
				let points_copy = points_clone.borrow().clone();

				clear_lines(canvas.clone(), ctx.clone());
				draw_circles(points_copy.clone(), ctx.clone());

				let bound_rect = canvas.get_bounding_client_rect();
				let cursor_x = ev.page_x() - bound_rect.left() as i32;
				let cursor_y = ev.page_y() - bound_rect.top() as i32;

				draw_lines(points_copy, ctx, cursor_x, cursor_y);
			}
		}
	};

	let on_mouse_out = {
		let points_ref = Rc::clone(&points);
		move |_| {
			if let Some((canvas, ctx)) = get_canvas_and_context() {
				let points = points_ref.borrow().clone();
				clear_lines(canvas, ctx.clone());
				draw_circles(points, ctx);
			}
		}
	};

	view! {
	<canvas
		node_ref=canvas_ref
		on:mousemove=on_mouse_move
		on:mouseout=on_mouse_out
		class=styles::canvas
	/>
	}
}

fn square(x: i32) -> i32 {
	x * x
}
