using System;

namespace Metrix.Models.Backend
{
    public class MetricRangeQuery
    {
        public MetricQuery MetricQuery { get; set; }
        public DateTime StartDate { get; set; }
        public DateTime EndDate { get; set; }
    }
}