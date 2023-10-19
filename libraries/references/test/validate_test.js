import assert from 'assert';
import { getCommonSchemas } from '../src/common-schemas.js';
import libUrls from 'taskcluster-lib-urls';
import _ from 'lodash';
import References from '../src/index.js';
import { validate } from '../src/validate.js';
import testing from 'taskcluster-lib-testing';

const { omit, merge } = _;

class RefBuilder {
  constructor() {
    this.schemas = [...getCommonSchemas()];
    this.references = [];
  }

  schema({ omitPaths = [], filename = 'test-schema.yml', ...content }) {
    this.schemas.push({
      filename,
      content: omit(merge({
        $schema: '/schemas/common/metaschema.json#',
        $id: '/schemas/test/test.json#',
      }, content), omitPaths),
    });
    return this;
  }

  apiref({ omitPaths = [], filename = 'test-api-ref.yml', entries = [], ...content }) {
    this.references.push({
      filename,
      content: omit(merge({
        $schema: '/schemas/common/api-reference-v0.json#',
        apiVersion: 'v2',
        serviceName: 'test',
        title: 'Test Service',
        description: 'Test Service',
        entries: entries.map(({ omitPaths = [], ...content }) => omit(merge({
          type: 'function',
          name: 'foo',
          title: 'Foo',
          description: 'Foo-bar',
          category: 'Foo',
          method: 'get',
          route: '/foo',
          args: [],
          stability: 'experimental',
        }, content), omitPaths)),
      }, content), omitPaths),
    });
    return this;
  }

  exchangesref({ omitPaths = [], filename = 'test-exch-ref.yml', entries = [], ...content }) {
    this.references.push({
      filename,
      content: omit(merge({
        $schema: '/schemas/common/exchanges-reference-v0.json#',
        apiVersion: 'v2',
        serviceName: 'test',
        title: 'Test Service',
        description: 'Test Service',
        exchangePrefix: 'test/v2',
        entries: entries.map(({ omitPaths = [], ...content }) => omit(merge({
          type: 'topic-exchange',
          exchange: 'test',
          name: 'foo',
          title: 'Foo',
          description: 'Foo-bar',
          routingKey: [],
          schema: 'v2/message.json#',
        }, content), omitPaths)),
      }, content), omitPaths),
    });
    return this;
  }

  end() {
    return new References(this).asAbsolute(libUrls.testRootUrl());
  }
}

suite(testing.suiteName(), function() {
  const assertProblems = (references, expected) => {
    try {
      validate(references);
    } catch (e) {
      if (!expected.length || !e.problems) {
        throw e;
      }
      assert.deepEqual(e.problems.sort(), expected.sort());
      return;
    }
    if (expected.length) {
      throw new Error('Expected problems not identified');
    }
  };

  test('empty references pass', function() {
    const references = new RefBuilder().end();
    assertProblems(references, []);
  });

  test('schema with no $id fails', function() {
    const references = new RefBuilder()
      .schema({ omitPaths: ['$id'] })
      .end();
    assertProblems(references, ['schema test-schema.yml has no $id']);
  });

  test('schema with invalid $id fails', function() {
    const references = new RefBuilder()
      .schema({ $id: '/schemas/foo.yml' })
      .end();
    assertProblems(references, [
      'schema test-schema.yml has an invalid $id \'https://tc-tests.example.com/schemas/foo.yml\' ' +
      '(expected \'/schemas/<something>/something>.json#\'',
    ]);
  });

  test('schema with invalid absolute $ref fails', function() {
    const references = new RefBuilder()
      .schema({
        type: 'object',
        properties: {
          foo: { $ref: 'https://example.com/schema.json#' },
        },
      })
      .end();
    assertProblems(references, [
      'schema test-schema.yml $ref at schema.properties.foo is not allowed',
    ]);
  });

  test('schema with invalid relative $ref fails', function() {
    const references = new RefBuilder()
      .schema({
        type: 'object',
        properties: {
          foo: { $ref: '../uncommon/foo.json#' },
        },
      })
      .end();
    assertProblems(references, [
      'schema test-schema.yml $ref at schema.properties.foo is not allowed',
    ]);
  });

  test('schema with no metaschema fails', function() {
    const references = new RefBuilder()
      .schema({ omitPaths: ['$schema'] })
      .end();
    assertProblems(references, ['schema test-schema.yml has no $schema']);
  });

  test('common schema with custom metaschema passes', function() {
    const references = new RefBuilder()
      .schema({
        $id: '/schemas/common/some-format.json#',
        $schema: '/schemas/common/metaschema.json#',
        metadata: { name: 'api', version: 1 },
      })
      .end();
    assertProblems(references, []);
  });

  test('invalid schema fails', function() {
    const references = new RefBuilder()
      .schema({
        type: 'object',
        additionalProperties: false,
        required: ['abc'],
        properties: {
          abc: ['a'],
        },
      })
      .end();
    assertProblems(references, [
      'test-schema.yml: schema/properties/abc should be object,boolean',
    ]);
  });

  test('schema with "entries" but no "type" fails', function() {
    const references = new RefBuilder()
      .schema({
        entries: { type: 'string' },
        uniqueItems: true,
      })
      .end();
    assertProblems(references, [
      'test-schema.yml: schema has a \'entries\' property but no \'type\'',
    ]);
  });

  test('schema with "entries" but no "uniqueItems" fails', function() {
    const references = new RefBuilder()
      .schema({
        type: 'array',
        entries: { type: 'string' },
      })
      .end();
    assertProblems(references, [
      'test-schema.yml: schema has a \'entries\' property but no \'uniqueItems\'',
    ]);
  });

  test('schema with "properties" but no "type" fails', function() {
    const references = new RefBuilder()
      .schema({
        properties: {},
        additionalProperties: false,
      })
      .end();
    assertProblems(references, [
      'test-schema.yml: schema has a \'properties\' property but no \'type\'',
    ]);
  });

  test('schema with "properties" but no "additionalProperties" fails', function() {
    const references = new RefBuilder()
      .schema({
        type: 'object',
        properties: {},
      })
      .end();
    assertProblems(references, [
      'test-schema.yml: schema has a \'properties\' property but no \'additionalProperties\'',
    ]);
  });

  test('invalid schema with custom metaschema fails', function() {
    const references = new RefBuilder()
      .schema({
        $schema: '/schemas/common/metadata-metaschema.json#',
        metadata: { version: 1 },
      })
      .end();
    assertProblems(references, [
      'test-schema.yml: schema/metadata should have required property \'name\'',
    ]);
  });

  test('schema with undefined metaschema fails', function() {
    const references = new RefBuilder()
      .schema({ $schema: '/schemas/nosuch.json#' })
      .end();
    assertProblems(references, [
      'schema test-schema.yml has invalid $schema (must be defined here or be on at json-schema.org)',
    ]);
  });

  test('api reference with no $schema fails', function() {
    const references = new RefBuilder()
      .apiref({ omitPaths: ['$schema'] })
      .end();
    assertProblems(references, ['reference test-api-ref.yml has no $schema']);
  });

  test('exchanges reference with no $schema fails', function() {
    const references = new RefBuilder()
      .exchangesref({ omitPaths: ['$schema'] })
      .end();
    assertProblems(references, ['reference test-exch-ref.yml has no $schema']);
  });

  test('invalid api reference fails', function() {
    const references = new RefBuilder()
      .apiref({ serviceName: true })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: reference/serviceName should be string',
    ]);
  });

  test('invalid exchanges reference fails', function() {
    const references = new RefBuilder()
      .schema({ $id: '/schemas/test/v2/message.json#' })
      .exchangesref({ title: false })
      .end();
    assertProblems(references, [
      'test-exch-ref.yml: reference/title should be string',
    ]);
  });

  test('reference with undefined $schema fails', function() {
    const references = new RefBuilder()
      .apiref({ $schema: '/schemas/nosuch.json#' })
      .end();
    assertProblems(references, [
      'reference test-api-ref.yml has invalid $schema (must be defined here)',
    ]);
  });

  test('reference with non-metadata metaschema fails', function() {
    const references = new RefBuilder()
      .apiref({ $schema: '/schemas/common/metadata-metaschema.json#' })
      .end();
    assertProblems(references, [
      'reference test-api-ref.yml has schema ' +
      '\'https://tc-tests.example.com/schemas/common/metadata-metaschema.json#\' ' +
      'which does not have the metadata metaschema',
    ]);
  });

  test('exchanges reference with absolute entry schema URL fails', function() {
    const references = new RefBuilder()
      .exchangesref({ entries: [{ schema: 'https://schemas.exmaple.com/message.json#' }] })
      .end();
    assertProblems(references, [
      'test-exch-ref.yml: entries[0].schema is not relative to the service',
    ]);
  });

  test('exchanges reference with /-relative entry schema (that exists) fails', function() {
    const references = new RefBuilder()
      .schema({ $id: '/schemas/test/v2/message.json#' })
      .exchangesref({ entries: [{ schema: '/schemas/test/v2/message.json#' }] })
      .end();
    assertProblems(references, [
      'test-exch-ref.yml: entries[0].schema is not relative to the service',
    ]);
  });

  test('exchanges reference with ../-relative entry schema (that exists) fails', function() {
    const references = new RefBuilder()
      .schema({ $id: '/schemas/test/v2/message.json#' })
      .exchangesref({ entries: [{ schema: '../test/v2/message.json#' }] })
      .end();
    assertProblems(references, [
      'test-exch-ref.yml: entries[0].schema is not relative to the service',
    ]);
  });

  test('exchanges reference with entry schema that does not exist fails', function() {
    const references = new RefBuilder()
      .exchangesref({ entries: [{ schema: 'v2/message.json#' }] })
      .end();
    assertProblems(references, [
      'test-exch-ref.yml: entries[0].schema does not exist',
    ]);
  });

  test('api reference with absolute entry input URL fails', function() {
    const references = new RefBuilder()
      .apiref({ entries: [{ input: 'https://schemas.exmaple.com/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: entries[0].input is not relative to the service',
    ]);
  });

  test('api reference with /-relative entry input (that exists) fails', function() {
    const references = new RefBuilder()
      .schema({ $id: '/schemas/test/v2/resource.json#' })
      .apiref({ entries: [{ input: '/schemas/test/v2/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: entries[0].input is not relative to the service',
    ]);
  });

  test('api reference with entry input that does not exist fails', function() {
    const references = new RefBuilder()
      .apiref({ entries: [{ input: 'v2/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: entries[0].input does not exist',
    ]);
  });

  test('api reference with absolute entry output URL fails', function() {
    const references = new RefBuilder()
      .apiref({ entries: [{ output: 'https://schemas.exmaple.com/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: entries[0].output is not relative to the service',
    ]);
  });

  test('api reference with /-relative entry output (that exists) fails', function() {
    const references = new RefBuilder()
      .schema({ $id: '/schemas/test/v2/resource.json#' })
      .apiref({ entries: [{ output: '/schemas/test/v2/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: entries[0].output is not relative to the service',
    ]);
  });

  test('api reference with entry output that does not exist fails', function() {
    const references = new RefBuilder()
      .apiref({ entries: [{ output: 'v2/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test-api-ref.yml: entries[0].output does not exist',
    ]);
  });

  test('api reference with entry output that exists but has wrong $schema fails', function() {
    const references = new RefBuilder()
      .schema({
        $schema: 'http://json-schema.org/draft-06/schema#',
        $id: '/schemas/test/v2/resource.json#',
      })
      .apiref({ entries: [{ output: 'v2/resource.json#' }] })
      .end();
    assertProblems(references, [
      'test/v2/resource.json#\'s $schema is not the metaschema',
    ]);
  });

  test('api reference with entry output that exists but has right $schema passes', function() {
    const references = new RefBuilder()
      .schema({
        $schema: 'https://tc-tests.example.com/schemas/common/metaschema.json#',
        $id: '/schemas/test/v2/resource.json#',
      })
      .apiref({ entries: [{ output: 'v2/resource.json#' }] })
      .end();
    assertProblems(references, []);
  });

  test('service schema referenced by service passes', function() {
    const references = new RefBuilder()
      .schema({
        $id: '/schemas/test/test.json#',
      })
      .apiref({
        entries: [{ input: 'test.json#' }],
      })
      .end();
    assertProblems(references, []);
  });

  test('service schema *not* referenced by service fails', function() {
    const references = new RefBuilder()
      .schema({
        $id: '/schemas/test/test.json#',
      })
      .end();
    assertProblems(references, [
      'schema https://tc-tests.example.com/schemas/test/test.json# not referenced anywhere',
    ]);
  });
});
