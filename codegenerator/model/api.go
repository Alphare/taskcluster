package model

import (
	"fmt"
	"strings"

	"github.com/taskcluster/taskcluster-client-go/codegenerator/utils"
)

//////////////////////////////////////////////////////////////////
//
// From: http://schemas.taskcluster.net/base/v1/api-reference.json
//
//////////////////////////////////////////////////////////////////

type API struct {
	Version     interface{} `json:"version"`
	Title       string      `json:"title"`
	Description string      `json:"description"`
	BaseURL     string      `json:"baseUrl"`
	Entries     []APIEntry  `json:"entries"`

	apiDef *APIDefinition
}

func (api *API) String() string {
	var result string = fmt.Sprintf(
		"Version     = '%v'\n"+
			"Title       = '%v'\n"+
			"Description = '%v'\n"+
			"Base URL    = '%v'\n",
		api.Version, api.Title, api.Description, api.BaseURL)
	for i, entry := range api.Entries {
		result += fmt.Sprintf("Entry %-6v=\n%v", i, entry.String())
	}
	return result
}

func (api *API) postPopulate(apiDef *APIDefinition) {

	// make sure each entry defined for this API has a unique generated method name
	methods := make(map[string]bool)

	for i := range api.Entries {
		api.Entries[i].Parent = api
		api.Entries[i].MethodName = utils.Normalise(api.Entries[i].Name, methods)
		api.Entries[i].postPopulate(apiDef)
	}
}

func (api *API) generateAPICode(apiName string) string {
	// package name and variable name cannot be the same
	// so find a way to make them different...
	// e.g. switch case of first character, and if first
	// character is not can't switch case for whatever
	// reason, prefix variable name with "my"
	var exampleVarName string
	switch firstChar := string(api.apiDef.PackageName[0]); {
	case strings.ToUpper(firstChar) != firstChar:
		exampleVarName = strings.ToUpper(firstChar) + api.apiDef.PackageName[1:]
	case strings.ToLower(firstChar) != firstChar:
		exampleVarName = strings.ToLower(firstChar) + api.apiDef.PackageName[1:]
	default:
		exampleVarName = "my" + api.apiDef.PackageName
	}
	exampleCall := ""
	// here we choose an example API method to call, just the first one in the list of api.Entries
	// We need to first see if it returns one or two variables...
	if api.Entries[0].Output == "" {
		exampleCall = "//  callSummary := " + exampleVarName + "." + api.Entries[0].MethodName + "(.....)"
	} else {
		exampleCall = "//  data, callSummary := " + exampleVarName + "." + api.Entries[0].MethodName + "(.....)"
	}
	comment := ""
	if api.Description != "" {
		comment = utils.Indent(api.Description, "// ")
	}
	if len(comment) >= 1 && comment[len(comment)-1:] != "\n" {
		comment += "\n"
	}
	comment += "//\n"
	comment += fmt.Sprintf("// See: %v\n", api.apiDef.DocRoot)
	comment += "//\n"
	comment += "// How to use this package\n"
	comment += "//\n"
	comment += "// First create an authentication object:\n"
	comment += "//\n"
	comment += "//  " + exampleVarName + " := " + api.apiDef.PackageName + ".New(\"myClientId\", \"myAccessToken\")\n"
	comment += "//\n"
	comment += "// and then call one or more of auth's methods, e.g.:\n"
	comment += "//\n"
	comment += exampleCall + "\n"
	comment += "// handling any errors...\n"
	comment += "//  if callSummary.Error != nil {\n"
	comment += "//  	// handle error...\n"
	comment += "//  }\n"

	content := comment
	content += "package " + api.apiDef.PackageName + "\n"
	content += `
import (
	"bytes"
	"crypto/sha256"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"reflect"
	"strconv"
	"time"
	"github.com/cenkalti/backoff"
	hawk "github.com/tent/hawk-go"
	D "github.com/tj/go-debug"
%%{imports}
)

var (
	// These defaults *approximate* to the node.js taskcluster client settings.
	BackOffSettings *backoff.ExponentialBackOff = &backoff.ExponentialBackOff {
		InitialInterval:     100 * time.Millisecond,
		RandomizationFactor: 0.2,
		Multiplier:          2,
		MaxInterval:         30 * time.Second,
		MaxElapsedTime:      60 * time.Second,
		Clock:               backoff.SystemClock,
	}
	// Used for logging based on DEBUG environment variable
	// See github.com/tj/go-debug
	debug = D.Debug("` + api.apiDef.PackageName + `")
)

// apiCall is the generic REST API calling method which performs all REST API
// calls for this library.  Each auto-generated REST API method simply is a
// wrapper around this method, calling it with specific specific arguments.
func (auth *Auth) apiCall(payload interface{}, method, route string, result interface{}) (interface{}, *CallSummary) {
	callSummary := new(CallSummary)
	callSummary.HttpRequestObject = payload
	jsonPayload, err := json.Marshal(payload)
	if err != nil {
		callSummary.Error = err
		return result, callSummary
	}
	callSummary.HttpRequestBody = string(jsonPayload)

	httpClient := &http.Client{}

	// function to perform http request - we call this using backoff library to
	// have exponential backoff in case of intermittent failures (e.g. network
	// blips)
	httpCall := func() error {
		var ioReader io.Reader = nil
		if reflect.ValueOf(payload).IsValid() && !reflect.ValueOf(payload).IsNil() {
			ioReader = bytes.NewReader(jsonPayload)
		}
		httpRequest, err := http.NewRequest(method, auth.BaseURL+route, ioReader)
		if err != nil {
			callSummary.Error = fmt.Errorf("apiCall url cannot be parsed: '%v', is your BaseURL (%v) set correctly?", auth.BaseURL+route, auth.BaseURL)
			return nil
		}
		httpRequest.Header.Set("Content-Type", "application/json")
		callSummary.HttpRequest = httpRequest
		debug("Making http reqest: %v", httpRequest)
		// Refresh Authorization header with each call...
		// Only authenticate if client library user wishes to.
		if auth.Authenticate {
			credentials := &hawk.Credentials{
				ID:   auth.ClientId,
				Key:  auth.AccessToken,
				Hash: sha256.New,
			}
			reqAuth := hawk.NewRequestAuth(httpRequest, credentials, 0).RequestHeader()
			httpRequest.Header.Set("Authorization", reqAuth)
		}
		response, err := httpClient.Do(httpRequest)
		if err != nil {
			return err
		}
		callSummary.HttpResponse = response
		defer response.Body.Close()
		// now read response into memory, so that we can return the body
		body, err := ioutil.ReadAll(response.Body)
		if err != nil {
			return err
		}
		callSummary.HttpResponseBody = string(body)

		// now check if http response code is such that we should retry [500, 600)...
		if respCode := response.StatusCode; respCode/100 == 5 {
			return BadHttpResponseCode{
				HttpResponseCode: respCode,
				Message: "(Intermittent) HTTP response code " + strconv.Itoa(respCode) + " to http " +
					method + " request to " + auth.BaseURL + route + ":\n" +
					callSummary.HttpResponseBody + "\n",
			}
		}

		return nil
	}

	// Make HTTP API calls using an exponential backoff algorithm...
	callSummary.Error = backoff.RetryNotify(httpCall, BackOffSettings, func(err error, wait time.Duration) {
		debug("Error: %s", err)
		callSummary.Attempts += 1
	})

	if callSummary.Error != nil {
		return result, callSummary
	}

	// now check http response code is ok [200, 300)...
	if respCode := callSummary.HttpResponse.StatusCode; respCode/100 != 2 {
		callSummary.Error = BadHttpResponseCode{
			HttpResponseCode: respCode,
			Message: "(Permanent) HTTP response code " + strconv.Itoa(respCode) + " to http " +
				method + " request to " + auth.BaseURL + route + " (NOT retrying):\n" +
				callSummary.HttpResponseBody + "\n",
		}
		return result, callSummary
	}

	// if result is passed in as nil, it means the API defines no response body
	// json
	if reflect.ValueOf(result).IsValid() && !reflect.ValueOf(result).IsNil() {
		err = json.Unmarshal([]byte(callSummary.HttpResponseBody), &result)
		if err != nil {
			callSummary.Error = err
			// technically not needed, but more comprehensible
			return result, callSummary
		}
	}

	// Return result and callSummary
	return result, callSummary
}
`
	content += `
// The entry point into all the functionality in this package is to create an
// Auth object.  It contains your authentication credentials, which are
// required for all HTTP operations.
type Auth struct {
	// Client ID required by Hawk
	ClientId string
	// Access Token required by Hawk
	AccessToken string
	// The URL of the API endpoint to hit.
	// Use ` + "\"" + api.BaseURL + "\"" + ` for production.
	// Please note calling auth.New(clientId string, accessToken string) is an
	// alternative way to create an Auth object with BaseURL set to production.
	BaseURL string
	// Whether authentication is enabled (e.g. set to 'false' when using taskcluster-proxy)
	// Please note calling auth.New(clientId string, accessToken string) is an
	// alternative way to create an Auth object with Authenticate set to true.
	Authenticate bool
}

// CallSummary provides information about the underlying http request and
// response issued for a given API call, together with details of any Error
// which occured. After making an API call, be sure to check the returned
// CallSummary.Error - if it is nil, no error occurred.
type CallSummary struct {
	HttpRequest *http.Request
	// Keep a copy of request body in addition to the *http.Request, since
	// accessing the Body via the *http.Request object, you get a io.ReadCloser
	// - and after the request has been made, the body will have been read, and
	// the data lost... This way, it is still available after the api call
	// returns.
	HttpRequestBody string
	// The Go Type which is marshaled into json and used as the http request
	// body.
	HttpRequestObject interface{}
	HttpResponse      *http.Response
	// Keep a copy of response body in addition to the *http.Response, since
	// accessing the Body via the *http.Response object, you get a
	// io.ReadCloser - and after the response has been read once (to unmarshal
	// json into native go types) the data is lost... This way, it is still
	// available after the api call returns.
	HttpResponseBody string
	Error            error
	// Keep a record of how many http requests were attempted
	Attempts int
}

type BadHttpResponseCode struct {
	HttpResponseCode int
	Message          string
}

func (err BadHttpResponseCode) Error() string {
	return err.Message
}

// Returns a pointer to Auth, configured to run against production.  If you
// wish to point at a different API endpoint url, set BaseURL to the preferred
// url. Authentication can be disabled (for example if you wish to use the
// taskcluster-proxy) by setting Authenticate to false.
//
`
	content += "// For example:\n"
	content += "//  " + exampleVarName + " := " + api.apiDef.PackageName + ".New(\"123\", \"456\") " + strings.Repeat(" ", 20+len(apiName)-len(api.apiDef.PackageName)) + "  // set clientId and accessToken\n"
	content += "//  " + exampleVarName + ".Authenticate = false             " + strings.Repeat(" ", len(apiName)) + "           // disable authentication (true by default)\n"
	content += "//  " + exampleVarName + ".BaseURL = \"http://localhost:1234/api/" + apiName + "/v1\"   // alternative API endpoint (production by default)\n"
	content += exampleCall + strings.Repeat(" ", 48-len(exampleCall)+len(apiName)+len(exampleVarName)) + " // for example, call the " + api.Entries[0].MethodName + "(.....) API endpoint (described further down)...\n"
	content += "//  if callSummary.Error != nil {\n"
	content += "//  	// handle errors...\n"
	content += "//  }\n"
	content += "func New(clientId string, accessToken string) *Auth {\n"
	content += "\treturn &Auth{\n"
	content += "\t\tClientId: clientId,\n"
	content += "\t\tAccessToken: accessToken,\n"
	content += "\t\tBaseURL: \"" + api.BaseURL + "\",\n"
	content += "\t\tAuthenticate: true}\n"
	content += "}\n"
	content += "\n"
	for _, entry := range api.Entries {
		content += entry.generateAPICode(apiName)
	}
	return content
}

func (api *API) setAPIDefinition(apiDef *APIDefinition) {
	api.apiDef = apiDef
}

type APIEntry struct {
	Type        string     `json:"type"`
	Method      string     `json:"method"`
	Route       string     `json:"route"`
	Args        []string   `json:"args"`
	Name        string     `json:"name"`
	Scopes      [][]string `json:"scopes"`
	Input       string     `json:"input"`
	Output      string     `json:"output"`
	Title       string     `json:"title"`
	Description string     `json:"description"`

	MethodName string
	Parent     *API
}

func (entry *APIEntry) postPopulate(apiDef *APIDefinition) {
	if entry.Input != "" {
		entry.Parent.apiDef.cacheJsonSchema(&entry.Input)
		entry.Parent.apiDef.schemas[entry.Input].IsInputSchema = true
	}
	if entry.Output != "" {
		entry.Parent.apiDef.cacheJsonSchema(&entry.Output)
		entry.Parent.apiDef.schemas[entry.Output].IsOutputSchema = true
	}
}

func (entry *APIEntry) String() string {
	return fmt.Sprintf(
		"    Entry Type        = '%v'\n"+
			"    Entry Method      = '%v'\n"+
			"    Entry Route       = '%v'\n"+
			"    Entry Args        = '%v'\n"+
			"    Entry Name        = '%v'\n"+
			"    Entry Scopes      = '%v'\n"+
			"    Entry Input       = '%v'\n"+
			"    Entry Output      = '%v'\n"+
			"    Entry Title       = '%v'\n"+
			"    Entry Description = '%v'\n",
		entry.Type, entry.Method, entry.Route, entry.Args,
		entry.Name, entry.Scopes, entry.Input, entry.Output,
		entry.Title, entry.Description)
}

func (entry *APIEntry) generateAPICode(apiName string) string {
	comment := ""
	if entry.Description != "" {
		comment = utils.Indent(entry.Description, "// ")
	}
	if len(comment) >= 1 && comment[len(comment)-1:] != "\n" {
		comment += "\n"
	}
	comment += "//\n"
	comment += fmt.Sprintf("// See %v/#%v\n", entry.Parent.apiDef.DocRoot, entry.Name)
	inputParams := ""
	if len(entry.Args) > 0 {
		inputParams += strings.Join(entry.Args, " string, ") + " string"
	}

	apiArgsPayload := "nil"
	if entry.Input != "" {
		apiArgsPayload = "payload"
		p := "payload *" + entry.Parent.apiDef.schemas[entry.Input].TypeName
		if inputParams == "" {
			inputParams = p
		} else {
			inputParams += ", " + p
		}
	}

	responseType := "*CallSummary"
	if entry.Output != "" {
		responseType = "(*" + entry.Parent.apiDef.schemas[entry.Output].TypeName + ", *CallSummary)"
	}

	content := comment
	content += "func (a *Auth) " + entry.MethodName + "(" + inputParams + ") " + responseType + " {\n"
	if entry.Output != "" {
		content += "\tresponseObject, callSummary := a.apiCall(" + apiArgsPayload + ", \"" + strings.ToUpper(entry.Method) + "\", \"" + strings.Replace(strings.Replace(entry.Route, "<", "\" + ", -1), ">", " + \"", -1) + "\", new(" + entry.Parent.apiDef.schemas[entry.Output].TypeName + "))\n"
		content += "\treturn responseObject.(*" + entry.Parent.apiDef.schemas[entry.Output].TypeName + "), callSummary\n"
	} else {
		content += "\t_, callSummary := a.apiCall(" + apiArgsPayload + ", \"" + strings.ToUpper(entry.Method) + "\", \"" + strings.Replace(strings.Replace(entry.Route, "<", "\" + ", -1), ">", " + \"", -1) + "\", nil)\n"
		content += "\treturn callSummary\n"
	}
	content += "}\n"
	content += "\n"
	return content
}
