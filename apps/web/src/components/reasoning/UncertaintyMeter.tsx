interface UncertaintyMeterProps {
	score: number;
	factors?: string[];
}

export default function UncertaintyMeter(props: UncertaintyMeterProps) {
	const score = () => Math.max(0, Math.min(1, props.score));
	return (
		<section class="uncertainty-meter" aria-label="Tingkat kepastian analisis">
			<div class="section-title">
				<h3>Kepastian awal</h3>
				<strong>{score().toFixed(2)}</strong>
			</div>
			<div class="progress" role="progressbar" aria-valuenow={score() * 100} aria-valuemin="0" aria-valuemax="100">
				<span style={{ width: `${score() * 100}%` }} />
			</div>
			{props.factors && props.factors.length > 0 && (
				<ul class="uncertainty-factors">
					{props.factors.map((factor) => <li>{factor}</li>)}
				</ul>
			)}
		</section>
	);
}
