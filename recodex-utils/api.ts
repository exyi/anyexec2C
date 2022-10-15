// deno-lint-ignore-file no-explicit-any


export function createApi({ token, baseUrl }: { token: string, baseUrl?: string }) {
	baseUrl ??= 'https://recodex.mff.cuni.cz/'
	baseUrl = baseUrl.replace(/\/*$/, '/')

	const defaultHeaders = { 'Authorization': 'Bearer ' + token }

	function listArchivedCourses(): Promise<RecodexResponse<RecodexCourse[]>> {
		return fetch(`${baseUrl}api/v1/groups?ancestors=1&archived=1`, { headers: defaultHeaders })
			.then(res => res.json())
	}

	function listAssignments(courseId: string): Promise<RecodexResponse<RecodexAssignment[]>> {
		return fetch(`${baseUrl}api/v1/groups/${courseId}/assignments`, { headers: defaultHeaders })
			.then(res => res.json())
	}

	function courseStats(courseId: string): Promise<RecodexResponse<RecodexCourseStats[]>> {
		return fetch(`${baseUrl}api/v1/groups/${courseId}/students/stats`, { headers: defaultHeaders })
			.then(res => res.json())
	}
	function getUsers(userIds: string[]): Promise<RecodexResponse<RecodexUserInfo[]>> {
		return fetch(`${baseUrl}api/v1/users/list`, { headers: { ...defaultHeaders, 'Content-Type': "application/json" }, method: "POST", body: JSON.stringify({ ids: userIds }) })
			.then(res => res.json())
	}

	function assignmentSubmissions(assignmentId: string, userId: string): Promise<RecodexResponse<AssignmentSubmission[]>> {
		return fetch(`${baseUrl}api/v1/exercise-assignments/${assignmentId}/users/${userId}/solutions`, { headers: defaultHeaders })
			.then(res => res.json())
	}

	function getSubmissionFiles(submissionId: string): Promise<RecodexResponse<SubmissionFile[]>> {
		return fetch(`${baseUrl}api/v1/assignment-solutions/${submissionId}/files`, { headers: defaultHeaders })
			.then(res => res.json())
	}

	function getFileContent(fileId: string): Promise<Blob> {
		return fetch(`${baseUrl}api/v1/uploaded-files/${fileId}/download`, { headers: defaultHeaders })
			.then(res => res.blob())
	}


	return {
		listArchivedCourses,
		listAssignments,
		courseStats,
		getUsers,
		assignmentSubmissions,
		getSubmissionFiles,
		getFileContent,
	}
}

type UnixTime = number

export type RecodexResponse<T> = {
	code: number
	success: boolean
	payload: T
}

export type LocalizedText<T> = {
	id: string
	locale: string
	createdAt: string
} & T

export type RecodexCourse = {
	/** Use for `listAssignments(this.id)` */
	id: string
	externalId: string
	organizational: boolean
	archived: boolean
	public: boolean
	directlyArchived: boolean
	localizedTexts: LocalizedText<{ name: string, description: string }>[]
	primaryAdminsIds: string[]
	parentGroupId: string
	parentGroupIds: string[]
	childGroups: string[]
	permissionHints: RecodexCoursePermissionHints
	/** WTF, it's public anyways... */
	privateData: CoursePrivateData
}

export type CoursePrivateData = {
	admins: string[]
	supervisors: string[]
	observers: string[]
	students: string[]
	instance: boolean
	assignments: string[]
	shadowAssignments: string[]
	publicStats: boolean
	detaining: boolean
	bindings: {
		/** Seznam rozvrhových lístků, např 20aNPFL129x02 */
		sis?: string[]
	}
}

export type RecodexCoursePermissionHints = {
	viewAssignments: boolean
	viewDetail: boolean
	viewSubgroups: boolean
	viewStudents: boolean
	viewMembers: boolean
	inviteStudents: boolean
	viewStats: boolean
	addSubgroup: boolean
	update: boolean
	remove: boolean
	archive: boolean
	relocate: boolean
	viewExercises: boolean
	assignExercise: boolean
	createExercise: boolean
	createShadowAssignment: boolean
	viewPublicDetail: boolean
	becomeMember: boolean
	sendEmail: boolean
	viewInvitations: boolean
	acceptInvitation: boolean
	editInvitations: boolean
}

export type RecodexAssignment = {
	id: string
	version: number
	isPublic: boolean
	createdAt: UnixTime
	updatedAt: UnixTime
	localizedTexts: LocalizedText<{ name: string, text: string, link: string, studentHint: string }>[]
	exerciseId: string
	groupId: string
	firstDeadline: UnixTime
	secondDeadline: UnixTime
	allowSecondDeadline: boolean
	maxPointsBeforeFirstDeadline: number
	maxPointsBeforeSecondDeadline: number
	maxPointsDeadlineInterpolation: false
	/** TODO: ? */
	visibleFrom: any
	submissionsCountLimit: number
	runtimeEnvironmentIds: string[]
	disabledRuntimeEnvironmentIds: string[]
	canViewLimitRatios: boolean, canViewJudgeStdout: boolean, canViewJudgeStderr: boolean, mergeJudgeLogs: boolean
	isBonus: boolean
	pointsPercentualThreshold: number
	solutionFilesLimit: null
	solutionSizeLimit: null
	exerciseSynchronizationInfo: any // probably useless
}

export type RecodexCourseStats = {
	userId: string
	groupId: string
	points: { total: number, gained: number }
	hasLimit: boolean
	passesLimit: boolean
	assignments: {
		id: string
		status: "done" | null
		points: { total: number, gained: number | null, bonus: number | null }
		bestSolutionId: string | null
		accepted: boolean
	}[]
}

export type AssignmentSubmission = {
	id: string
	/** From 1 */
	attemptIndex: number
	node: string
	assignmentId: string
	userId: string
	createdAt: UnixTime
	runtimeEnvironmentId: string
	maxPoints: number
	accepted: boolean
	reviewed: boolean
	isBestSolution: boolean
	actualPoints: number
	bonusPoints: number
	overriddenPoints: number | null
	submissions: string[]
	lastSubmission: {
		id: string
		assignmentSolutionId: string
		evaluationStatus: "done" | "failed" | "TODO"
		isCorrect: boolean
		submittedBy: string
		submittedAt: UnixTime
		isDebug: boolean

		evaluation: {
			id: string
			evaluatedAt: UnixTime
			score: number
			points: number
			initFailed: boolean
			/** Compiler output */
			initiationOutputs: string
			testResults: {
				id: number
				testName: string
				score: number
				// TODO
			}[]
		}


	}
	commentStats: any // TODO?
	permissionHints: any
}

export type SubmissionFile = {
	id: string
	name: string
	size: number
	uploadedAt: UnixTime
	userId: string
	isPublic: boolean
}

export type RecodexUserInfo = {
	id: string
	fullName: string
	avatarUrl: string | null
	isVerified: boolean
	/** Normally null, only visible for the current user (unless you are admin or something) */
	privateData: null | {
		email: string
		createdAt: UnixTime
	}
}
