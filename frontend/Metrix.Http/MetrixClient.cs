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
        private static readonly string BASE_URL = "http://localhost:8000";
        private static readonly string FORMAT_STRING = "y-m-dTh:m:s";
        private readonly HttpClient _client;
        private readonly CancellationTokenSource _cts;

        public MetrixClient(HttpClient client, CancellationTokenSource cts)
        {
            _client = client;
            _cts = cts;
        }

        public async Task PostMetric(MetricInsertPartial model)
        {
            await _client.PostAsync($"{BASE_URL}/metric/insert", new StringContent(JsonConvert.SerializeObject(model)),
                _cts.Token);
        }

        public async Task<MetricResult> GetMetric(MetricQuery query)
        {
            var ps = new Dictionary<string, string>
            {
                { "data_point", query.DataPoint },
                { "data_group", query.DataGroup }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BASE_URL}/metric", ps), _cts.Token);
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
                { "data_group", query.MetricQuery.DataGroup },
                { "date", query.Date.ToString(FORMAT_STRING) }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BASE_URL}/metric/history", ps),
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
                { "data_group", query.MetricQuery.DataGroup },
                { "start_date", query.StartDate.ToString(FORMAT_STRING) },
                { "end_date", query.StartDate.ToString(FORMAT_STRING) }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BASE_URL}/metric/history", ps),
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
            var result = await _client.GetAsync($"{BASE_URL}/metric/data-groupings", _cts.Token);
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
                { "data_grouping", grouping }
            };
            var result = await _client.GetAsync(QueryHelpers.AddQueryString($"{BASE_URL}/metric/data-points", ps),
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