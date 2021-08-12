using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using Metrix.Models;
using Metrix.Utils;
using Microsoft.AspNetCore.WebUtilities;
using Newtonsoft.Json;

namespace Metrix.Http
{
    public class MetrixClient
    {
        private static readonly string BaseUrl = "http://localhost:8000";
        private static readonly string FormatString = "yyyy-MM-ddTHH:mm:ss";
        private readonly HttpClient _client;
        private readonly CancellationTokenSource _cts;

        public MetrixClient(HttpClient client, CancellationTokenSource cts)
        {
            _client = client;
            _cts = cts;
        }

        public async Task PostMetric(MetricInsertPartial model)
        {
            await _client.PostAsync($"{BaseUrl}/metric/insert", new StringContent(JsonConvert.SerializeObject(model)),
                _cts.Token);
        }

        public async Task<MetricResult> GetMetric(MetricQuery query)
        {
            var ps = new Dictionary<string, string>
            {
                { "data_point", query.DataPoint },
                { "data_group", query.DataGrouping }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BaseUrl}/metric", ps), _cts.Token);
            var body = await result.Content.ReadAsStringAsync(_cts.Token);
            var deserialized = JsonConvert.DeserializeObject<MetricResult>(body, new JsonSerializerSettings
            {
                ContractResolver = UtilityMethods.GetDefaultJsonContractResolver()
            });
            return deserialized;
        }

        public async Task<List<MetricResult>> GetMetricHistory(MetricPointQuery query)
        {
            var ps = new Dictionary<string, string>
            {
                { "data_point", query.MetricQuery.DataPoint },
                { "data_group", query.MetricQuery.DataGrouping },
                { "date", query.Date.ToString(FormatString) }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BaseUrl}/metric/history", ps),
                _cts.Token);
            var body = await result.Content.ReadAsStringAsync(_cts.Token);
            var deserialized = JsonConvert.DeserializeObject<List<MetricResult>>(body, new JsonSerializerSettings
            {
                ContractResolver = UtilityMethods.GetDefaultJsonContractResolver()
            });
            return deserialized;
        }

        public async Task<List<MetricResult>> GetMetricSeries(MetricRangeQuery query)
        {
            var ps = new Dictionary<string, string>
            {
                { "data_point", query.MetricQuery.DataPoint },
                { "data_group", query.MetricQuery.DataGrouping },
                { "start_date", query.StartDate.ToString(FormatString) },
                { "end_date", query.EndDate.ToString(FormatString) }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BaseUrl}/metric/series", ps),
                _cts.Token);
            var body = await result.Content.ReadAsStringAsync(_cts.Token);
            var deserialized = JsonConvert.DeserializeObject<List<MetricResult>>(body, new JsonSerializerSettings
            {
                ContractResolver = UtilityMethods.GetDefaultJsonContractResolver()
            });
            return deserialized;
        }

        public async Task<List<string>> GetMetricDataGroupings()
        {
            var result = await _client.GetAsync($"{BaseUrl}/metric/data-groupings", _cts.Token);
            var body = await result.Content.ReadAsStringAsync(_cts.Token);
            var deserialized = JsonConvert.DeserializeObject<List<string>>(body, new JsonSerializerSettings
            {
                ContractResolver = UtilityMethods.GetDefaultJsonContractResolver()
            });
            return deserialized;
        }

        public async Task<List<string>> GetMetricDataPointsForGrouping(string grouping)
        {
            var ps = new Dictionary<string, string>
            {
                { "data_grouping", grouping ?? string.Empty }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BaseUrl}/metric/data-points", ps),
                _cts.Token);
            var body = await result.Content.ReadAsStringAsync(_cts.Token);
            var deserialized = JsonConvert.DeserializeObject<List<string>>(body, new JsonSerializerSettings
            {
                ContractResolver = UtilityMethods.GetDefaultJsonContractResolver()
            });
            return deserialized;
        }
    }
}