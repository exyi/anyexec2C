#!/usr/bin/env -S deno run --allow-net --allow-env --allow-write --allow-read
import { parse } from "https://deno.land/std@0.119.0/flags/mod.ts";
import { createApi, RecodexResponse } from "./api.ts";

const flags = parse(Deno.args, {
	boolean: [],
	string: ["token", "path"],
	default: { path: "recodex-archive" },
});


const token = flags.token

if (!token) {
	console.error("Missing --token BEARER_TOKEN")
	console.error("You can get it from the network tab of your browser when you are logged in to ReCodex, or by looking for \"jwt\": in ReCodex HTML.")
	Deno.exit(1)
}

function unwrap<T>(msg: string, res: RecodexResponse<T>): T {
	if (!res.success) {
		console.error(`API call ${msg} failed:`, res)
		Deno.exit(1)
	}
	return res.payload
}

const api = createApi({ token })

const courses = unwrap("List courses", await api.listArchivedCourses())

for (const course of courses) {
	if (!course.permissionHints.viewAssignments) {
		console.log(`Skipping ${course.externalId}: ${course.localizedTexts[0].name}, no permission to view assignments`)
		continue
	}
	console.log(`Listing ${course.externalId}: ${course.localizedTexts[0].name}`)
	const courseDir = `${flags.path}/${course.externalId ?? course.localizedTexts[0]?.name ?? course.id}`

	const assignments = unwrap("List assignments", await api.listAssignments(course.id))
	const stats = unwrap("Get course stats", await api.courseStats(course.id))
	const courseUsers = new Map(unwrap("Get users", await api.getUsers(stats.map(s => s.userId)))
		.map(u => [u.id, u] as [string, typeof u]))

	await Deno.mkdir(courseDir, { recursive: true })
	await Deno.writeTextFile(courseDir + "/assignments.json", JSON.stringify(assignments, null, '\t'))
	await Deno.writeTextFile(courseDir + "/users.json", JSON.stringify(courseUsers, null, '\t'))
	await Deno.writeTextFile(courseDir + "/stats.json", JSON.stringify(stats, null, '\t'))


	const statsByAId = new Map(
		stats.filter(s => courseUsers.get(s.userId)?.privateData != null)
			 .flatMap(s => s.assignments.map(a => [a.id, { userId: s.userId, ...a}] as [string, typeof a & { userId: string }])))
	for (const assignment of assignments) {
		const stat = statsByAId.get(assignment.id)
		if (!stat || stat.status == null) {
			// skip assignments without submissions
			continue
		}

		
		const assignmentDir = courseDir + "/" + assignment.localizedTexts[0].name
		
		await Deno.mkdir(assignmentDir, { recursive: true })
		await Deno.writeTextFile(assignmentDir + "/assignment.json", JSON.stringify(assignment, null, '\t'))
		
		const submissions = unwrap(
			`List submissions of ${assignment.localizedTexts[0].name} [${assignment.id}]`,
			await api.assignmentSubmissions(assignment.id, stat.userId))

		console.log(`  Loading ${course.externalId} / ${assignment.localizedTexts[0].name} (${submissions.length} submissions)`)

		await Deno.writeTextFile(assignmentDir + "/submissions.json", JSON.stringify(submissions, null, '\t'))

		for (const submission of submissions) {
			const submissionDir = assignmentDir + "/" + submission.attemptIndex
			await Deno.mkdir(submissionDir, { recursive: true })

			await Deno.writeTextFile(submissionDir + "/submission.json", JSON.stringify(submission, null, '\t'))

			const files = unwrap(
				`List files of ${assignment.localizedTexts[0].name} [${assignment.id}] submission #${submission.attemptIndex}`,
				await api.getSubmissionFiles(submission.id))

			for (const file of files) {
				const content = await api.getFileContent(file.id)

				await Deno.writeFile(submissionDir + "/" + file.name, new Uint8Array(await content.arrayBuffer()))
			}
		}
	}
}

