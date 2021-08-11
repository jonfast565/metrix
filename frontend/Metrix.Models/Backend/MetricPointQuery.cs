using System;

namespace Metrix.Models
{
    public class MetricPointQuery
    {
        public MetricQuery MetricQuery { get; set; }
        public DateTime Date { get; set; }
    }
}