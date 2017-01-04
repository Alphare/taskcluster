// The following code is AUTO-GENERATED. Please DO NOT edit.
// To update this generated code, run the following command:
// in the /codegenerator/model subdirectory of this project,
// making sure that `${GOPATH}/bin` is in your `PATH`:
//
// go install && go generate
//
// This package was generated from the schema defined at
// http://references.taskcluster.net/github/v1/exchanges.json

// The github service, typically available at
// `github.taskcluster.net`, is responsible for publishing a pulse
// message for supported github events.
//
// This document describes the exchange offered by the taskcluster
// github service
//
// See: https://docs.taskcluster.net/reference/core/github/exchanges
//
// How to use this package
//
// This package is designed to sit on top of http://godoc.org/github.com/taskcluster/pulse-go/pulse. Please read
// the pulse package overview to get an understanding of how the pulse client is implemented in go.
//
// This package provides two things in addition to the basic pulse package: structured types for unmarshaling
// pulse message bodies into, and custom Binding interfaces, for defining the fixed strings for task cluster
// exchange names, and routing keys as structured types.
//
// For example, when specifying a binding, rather than using:
//
//  pulse.Bind(
//  	"*.*.*.*.*.*.gaia.#",
//  	"exchange/taskcluster-queue/v1/task-defined")
//
// You can rather use:
//
//  queueevents.TaskDefined{WorkerType: "gaia"}
//
// In addition, this means that you will also get objects in your callback method like *queueevents.TaskDefinedMessage
// rather than just interface{}.
package githubevents

import (
	"reflect"
	"strings"
)

// When a GitHub pull request event is posted it will be broadcast on this
// exchange with the designated `organization` and `repository`
// in the routing-key along with event specific metadata in the payload.
//
// See https://docs.taskcluster.net/reference/core/github/exchanges#pullRequest
type PullRequest struct {
	RoutingKeyKind string `mwords:"*"`
	Organization   string `mwords:"*"`
	Repository     string `mwords:"*"`
	Action         string `mwords:"*"`
}

func (binding PullRequest) RoutingKey() string {
	return generateRoutingKey(&binding)
}

func (binding PullRequest) ExchangeName() string {
	return "exchange/taskcluster-github/v1/pull-request"
}

func (binding PullRequest) NewPayloadObject() interface{} {
	return new(GitHubPullRequestMessage)
}

// When a GitHub push event is posted it will be broadcast on this
// exchange with the designated `organization` and `repository`
// in the routing-key along with event specific metadata in the payload.
//
// See https://docs.taskcluster.net/reference/core/github/exchanges#push
type Push struct {
	RoutingKeyKind string `mwords:"*"`
	Organization   string `mwords:"*"`
	Repository     string `mwords:"*"`
}

func (binding Push) RoutingKey() string {
	return generateRoutingKey(&binding)
}

func (binding Push) ExchangeName() string {
	return "exchange/taskcluster-github/v1/push"
}

func (binding Push) NewPayloadObject() interface{} {
	return new(GitHubPushMessage)
}

// When a GitHub release event is posted it will be broadcast on this
// exchange with the designated `organization` and `repository`
// in the routing-key along with event specific metadata in the payload.
//
// See https://docs.taskcluster.net/reference/core/github/exchanges#release
type Release struct {
	RoutingKeyKind string `mwords:"*"`
	Organization   string `mwords:"*"`
	Repository     string `mwords:"*"`
}

func (binding Release) RoutingKey() string {
	return generateRoutingKey(&binding)
}

func (binding Release) ExchangeName() string {
	return "exchange/taskcluster-github/v1/release"
}

func (binding Release) NewPayloadObject() interface{} {
	return new(GitHubReleaseMessage)
}

func generateRoutingKey(x interface{}) string {
	val := reflect.ValueOf(x).Elem()
	p := make([]string, 0, val.NumField())
	for i := 0; i < val.NumField(); i++ {
		valueField := val.Field(i)
		typeField := val.Type().Field(i)
		tag := typeField.Tag
		if t := tag.Get("mwords"); t != "" {
			if v := valueField.Interface(); v == "" {
				p = append(p, t)
			} else {
				p = append(p, v.(string))
			}
		}
	}
	return strings.Join(p, ".")
}
