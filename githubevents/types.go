// This source code file is AUTO-GENERATED by github.com/taskcluster/jsonschema2go

package githubevents

import (
	"encoding/json"
)

type (
	// Message reporting that a GitHub pull request has occurred
	//
	// See http://schemas.taskcluster.net/github/v1/github-pull-request-message.json#
	GitHubPullRequestMessage struct {

		// The GitHub `action` which triggered an event.
		//
		// Possible values:
		//   * "assigned"
		//   * "unassigned"
		//   * "labeled"
		//   * "unlabeled"
		//   * "opened"
		//   * "edited"
		//   * "closed"
		//   * "reopened"
		//   * "synchronize"
		//
		// See http://schemas.taskcluster.net/github/v1/github-pull-request-message.json#/properties/action
		Action json.RawMessage `json:"action"`

		// Metadata describing the pull request.
		//
		// See http://schemas.taskcluster.net/github/v1/github-pull-request-message.json#/properties/details
		Details json.RawMessage `json:"details,omitempty"`

		// The GitHub `organization` which had an event.
		//
		// Syntax:     ^([a-zA-Z0-9-_%]*)$
		// Min length: 1
		// Max length: 100
		//
		// See http://schemas.taskcluster.net/github/v1/github-pull-request-message.json#/properties/organization
		Organization string `json:"organization"`

		// The GitHub `repository` which had an event.
		//
		// Syntax:     ^([a-zA-Z0-9-_%]*)$
		// Min length: 1
		// Max length: 100
		//
		// See http://schemas.taskcluster.net/github/v1/github-pull-request-message.json#/properties/repository
		Repository string `json:"repository"`

		// Message version
		//
		// Possible values:
		//   * 1
		//
		// See http://schemas.taskcluster.net/github/v1/github-pull-request-message.json#/properties/version
		Version json.RawMessage `json:"version"`
	}

	// Message reporting that a GitHub push has occurred
	//
	// See http://schemas.taskcluster.net/github/v1/github-push-message.json#
	GitHubPushMessage struct {

		// Metadata describing the push.
		//
		// See http://schemas.taskcluster.net/github/v1/github-push-message.json#/properties/details
		Details json.RawMessage `json:"details,omitempty"`

		// The GitHub `organization` which had an event.
		//
		// Syntax:     ^([a-zA-Z0-9-_%]*)$
		// Min length: 1
		// Max length: 100
		//
		// See http://schemas.taskcluster.net/github/v1/github-push-message.json#/properties/organization
		Organization string `json:"organization"`

		// The GitHub `repository` which had an event.
		//
		// Syntax:     ^([a-zA-Z0-9-_%]*)$
		// Min length: 1
		// Max length: 100
		//
		// See http://schemas.taskcluster.net/github/v1/github-push-message.json#/properties/repository
		Repository string `json:"repository"`

		// Message version
		//
		// Possible values:
		//   * 1
		//
		// See http://schemas.taskcluster.net/github/v1/github-push-message.json#/properties/version
		Version json.RawMessage `json:"version"`
	}

	// Message reporting that a GitHub release has occurred
	//
	// See http://schemas.taskcluster.net/github/v1/github-release-message.json#
	GitHubReleaseMessage struct {

		// Metadata describing the release.
		//
		// See http://schemas.taskcluster.net/github/v1/github-release-message.json#/properties/details
		Details json.RawMessage `json:"details,omitempty"`

		// The GitHub `organization` which had an event.
		//
		// Syntax:     ^([a-zA-Z0-9-_%]*)$
		// Min length: 1
		// Max length: 100
		//
		// See http://schemas.taskcluster.net/github/v1/github-release-message.json#/properties/organization
		Organization string `json:"organization"`

		// The GitHub `repository` which had an event.
		//
		// Syntax:     ^([a-zA-Z0-9-_%]*)$
		// Min length: 1
		// Max length: 100
		//
		// See http://schemas.taskcluster.net/github/v1/github-release-message.json#/properties/repository
		Repository string `json:"repository"`

		// Message version
		//
		// Possible values:
		//   * 1
		//
		// See http://schemas.taskcluster.net/github/v1/github-release-message.json#/properties/version
		Version json.RawMessage `json:"version"`
	}
)
